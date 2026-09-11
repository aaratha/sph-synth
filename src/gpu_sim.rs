use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::particles::Particle;

const WORKGROUP_SIZE: u32 = 64;

// Starting points for a 12x12 grid at 0.06 spacing in a [-0.9, 0.9] box — tune by eye.
const SMOOTHING_RADIUS: f32 = 0.15;
const REST_DENSITY: f32 = 10.0;
const STIFFNESS: f32 = 200.0;
const VISCOSITY: f32 = 1.5;
const PARTICLE_MASS: f32 = 1.0;
const GRAVITY_Y: f32 = 0.0;
const MOUSE_RADIUS: f32 = 0.25;
const MOUSE_STRENGTH: f32 = 2.0;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct PhysicsParams {
    dt: f32,
    gravity_y: f32,
    floor_y: f32,
    particle_count: u32,
    smoothing_radius: f32,
    rest_density: f32,
    stiffness: f32,
    viscosity: f32,
    particle_mass: f32,
    mouse_x: f32,
    mouse_y: f32,
    mouse_dx: f32,
    mouse_dy: f32,
    mouse_radius: f32,
    mouse_strength: f32,
    mouse_active: f32,
    // 16 fields * 4 bytes = 64, already a 16-byte multiple — no padding needed.
}

/// Runs physics.wgsl over the particle buffer each step. The buffer never leaves
/// the GPU: it's written once at construction, updated in place by the compute
/// shader, and read directly by Renderer::render() as the instance buffer.
pub struct GpuSim {
    device: wgpu::Device,
    queue: wgpu::Queue,
    particle_buffer: wgpu::Buffer,
    params_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    density_pipeline: wgpu::ComputePipeline,
    pressure_pipeline: wgpu::ComputePipeline,
    force_pipeline: wgpu::ComputePipeline,
    particle_count: u32,
    mouse_pos: [f32; 2],
    mouse_delta: [f32; 2],
    mouse_active: bool,
}

impl GpuSim {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue, initial: &[Particle]) -> Self {
        let particle_count = initial.len() as u32;

        let particle_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("particle buffer"),
            contents: bytemuck::cast_slice(initial),
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::VERTEX
                | wgpu::BufferUsages::COPY_DST,
        });

        let params_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("physics params buffer"),
            size: std::mem::size_of::<PhysicsParams>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("physics bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("physics bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: particle_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: params_buffer.as_entire_binding(),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("physics pipeline layout"),
            bind_group_layouts: &[Some(&bind_group_layout)],
            immediate_size: 0,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("physics shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("physics.wgsl").into()),
        });

        let make_pipeline = |entry_point: &'static str| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry_point),
                layout: Some(&pipeline_layout),
                module: &shader,
                entry_point: Some(entry_point),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                cache: None,
            })
        };
        let density_pipeline = make_pipeline("update_density");
        let pressure_pipeline = make_pipeline("update_pressure");
        let force_pipeline = make_pipeline("main");

        Self {
            device,
            queue,
            particle_buffer,
            params_buffer,
            bind_group,
            density_pipeline,
            pressure_pipeline,
            force_pipeline,
            particle_count,
            mouse_pos: [0.0, 0.0],
            mouse_delta: [0.0, 0.0],
            mouse_active: false,
        }
    }

    pub fn particle_buffer(&self) -> &wgpu::Buffer {
        &self.particle_buffer
    }

    pub fn particle_count(&self) -> u32 {
        self.particle_count
    }

    /// Sets the click-and-drag interaction: `world_pos` and `world_delta` (this
    /// frame's movement) are in the same [-1, 1] world space as particle
    /// positions. `active` is false while the mouse button is up.
    pub fn set_mouse(&mut self, world_pos: [f32; 2], world_delta: [f32; 2], active: bool) {
        self.mouse_pos = world_pos;
        self.mouse_delta = world_delta;
        self.mouse_active = active;
    }

    pub fn step(&self, dt: f32) {
        // Convert this frame's raw cursor displacement into a velocity, so the
        // push force stays consistent regardless of frame rate.
        let mouse_velocity = if dt > 1e-4 {
            [self.mouse_delta[0] / dt, self.mouse_delta[1] / dt]
        } else {
            [0.0, 0.0]
        };

        let params = PhysicsParams {
            dt,
            gravity_y: GRAVITY_Y,
            floor_y: -0.9,
            particle_count: self.particle_count,
            smoothing_radius: SMOOTHING_RADIUS,
            rest_density: REST_DENSITY,
            stiffness: STIFFNESS,
            viscosity: VISCOSITY,
            particle_mass: PARTICLE_MASS,
            mouse_x: self.mouse_pos[0],
            mouse_y: self.mouse_pos[1],
            mouse_dx: mouse_velocity[0],
            mouse_dy: mouse_velocity[1],
            mouse_radius: MOUSE_RADIUS,
            mouse_strength: MOUSE_STRENGTH,
            mouse_active: if self.mouse_active { 1.0 } else { 0.0 },
        };
        self.queue
            .write_buffer(&self.params_buffer, 0, bytemuck::bytes_of(&params));

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("physics step encoder"),
            });

        let workgroups = self.particle_count.div_ceil(WORKGROUP_SIZE);
        for pipeline in [&self.density_pipeline, &self.pressure_pipeline, &self.force_pipeline] {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("physics compute pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.dispatch_workgroups(workgroups, 1, 1);
        }

        self.queue.submit(Some(encoder.finish()));
    }
}
