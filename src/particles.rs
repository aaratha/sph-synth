use bytemuck::{Pod, Zeroable};

/// GPU-resident particle state — must match the `Particle` struct in physics.wgsl
/// field-for-field. Add density/pressure/etc. here (and in the shader) as SPH needs them.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Particle {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub density: f32,
    pub pressure: f32,
}

pub fn initial_grid(side: usize) -> Vec<Particle> {
    let mut particles = Vec::with_capacity(side * side);
    for i in 0..side {
        for j in 0..side {
            let x = (i as f32 - side as f32 / 2.0) * 0.06;
            let y = (j as f32 - side as f32 / 2.0) * 0.06 + 0.4;
            particles.push(Particle {
                position: [x, y],
                velocity: [0.0, 0.0],
                density: 1.0,
                pressure: 0.0
            });
        }
    }
    particles
}
