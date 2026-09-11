// Basic 2D SPH (Müller et al. 2003 style, brute-force O(n^2) neighbor search —
// no spatial hash yet, fine for a few hundred particles). Three passes per step,
// each a full dispatch over all particles, run in order within one encoder:
//   1. update_density  — accumulate density from all neighbors (poly6 kernel)
//   2. update_pressure — local equation of state, no neighbor read needed
//   3. main            — accumulate pressure + viscosity forces (spiky/viscosity
//                         kernels), add gravity, integrate, handle boundaries
// wgpu inserts the necessary storage-buffer barriers between passes automatically,
// so pass 2 always sees pass 1's writes and so on.

const PI: f32 = 3.14159265359;

struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
    density: f32,
    pressure: f32,
};

struct Params {
    dt: f32,
    gravity_y: f32,
    floor_y: f32,
    particle_count: u32,
    smoothing_radius: f32,
    rest_density: f32,
    stiffness: f32,
    viscosity: f32,
    particle_mass: f32,
};

@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;
@group(0) @binding(1) var<uniform> params: Params;

const WALL_X: f32 = 0.9;
const CEILING_Y: f32 = 0.9;
const BOUNCE_DAMPING: f32 = 0.4;

// Density contribution of a neighbor at squared distance r2, within smoothing radius h.
fn poly6_kernel(r2: f32, h: f32) -> f32 {
    if (r2 >= 0.0 && r2 <= h * h) {
        let coeff = 4.0 / (PI * pow(h, 8.0));
        let diff = h * h - r2;
        return coeff * diff * diff * diff;
    }
    return 0.0;
}

// Gradient of the spiky kernel, used for the (repulsive, non-singular-at-r=0) pressure force.
fn spiky_gradient(r_vec: vec2<f32>, r: f32, h: f32) -> vec2<f32> {
    if (r > 0.0 && r <= h) {
        let coeff = -30.0 / (PI * pow(h, 5.0));
        let diff = h - r;
        return coeff * diff * diff * (r_vec / r);
    }
    return vec2<f32>(0.0, 0.0);
}

// Laplacian of the viscosity kernel, used to smooth out relative velocities between neighbors.
fn viscosity_laplacian(r: f32, h: f32) -> f32 {
    if (r >= 0.0 && r <= h) {
        let coeff = 40.0 / (PI * pow(h, 5.0));
        return coeff * (h - r);
    }
    return 0.0;
}

@compute @workgroup_size(64)
fn update_density(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let i = global_id.x;
    if (i >= params.particle_count) {
        return;
    }

    let pos_i = particles[i].position;
    var density = 0.0;

    for (var j = 0u; j < params.particle_count; j++) {
        let r_vec = pos_i - particles[j].position;
        let r2 = dot(r_vec, r_vec);
        density += params.particle_mass * poly6_kernel(r2, params.smoothing_radius);
    }

    particles[i].density = density;
}

@compute @workgroup_size(64)
fn update_pressure(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let i = global_id.x;
    if (i >= params.particle_count) {
        return;
    }

    // Clamped ideal-gas equation of state: pressure only pushes particles apart,
    // never pulls them together.
    let pressure = params.stiffness * (particles[i].density - params.rest_density);
    particles[i].pressure = max(pressure, 0.0);
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let i = global_id.x;
    if (i >= params.particle_count) {
        return;
    }

    var p = particles[i];
    var force = vec2<f32>(0.0, params.gravity_y * p.density);

    for (var j = 0u; j < params.particle_count; j++) {
        if (j == i) {
            continue;
        }

        let other = particles[j];
        let r_vec = p.position - other.position;
        let r = length(r_vec);
        if (r >= params.smoothing_radius || r <= 0.0) {
            continue;
        }

        // Symmetric pressure force (Müller et al.), weighted by the spiky gradient.
        let pressure_term = (p.pressure / (p.density * p.density))
            + (other.pressure / (other.density * other.density));
        force -= params.particle_mass * pressure_term * spiky_gradient(r_vec, r, params.smoothing_radius);

        // Viscosity force pulls neighboring velocities toward each other.
        force += params.viscosity * params.particle_mass * (other.velocity - p.velocity)
            / other.density * viscosity_laplacian(r, params.smoothing_radius);
    }

    let acceleration = force / p.density;
    p.velocity += acceleration * params.dt;
    p.position += p.velocity * params.dt;

    if (p.position.y < params.floor_y) {
        p.position.y = params.floor_y;
        p.velocity.y *= -BOUNCE_DAMPING;
    }
    if (p.position.y > CEILING_Y) {
        p.position.y = CEILING_Y;
        p.velocity.y *= -BOUNCE_DAMPING;
    }
    if (p.position.x < -WALL_X) {
        p.position.x = -WALL_X;
        p.velocity.x *= -BOUNCE_DAMPING;
    }
    if (p.position.x > WALL_X) {
        p.position.x = WALL_X;
        p.velocity.x *= -BOUNCE_DAMPING;
    }

    particles[i] = p;
}
