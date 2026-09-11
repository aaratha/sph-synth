// GpuBackend struct + SphBackend impl

use sph_core::{ParticleSet, ParticleSnapshot, SphBackend, SphParams};

pub struct GpuBackend {
    device: wgpu::Device,
    queue: wgpu::Queue,
    // one wgpu::Buffer per SoA field — direct mirror of ParticleSet, no transpose needed
    position_buffer: wgpu::Buffer,
    velocity_buffer: wgpu::Buffer,
    density_buffer: wgpu::Buffer,
    pressure_buffer: wgpu::Buffer,
    mass_buffer: wgpu::Buffer,
    density_pipeline: wgpu::ComputePipeline,
    force_pipeline: wgpu::ComputePipeline,
    readback: ParticleSet, // reused scratch, avoids reallocating every snapshot()
}

impl GpuBackend {
    pub async fn new(initial: ParticleSet) -> Self {
        todo!()
    }
}

impl SphBackend for GpuBackend {
    fn step(&mut self, dt: f32, params: &SphParams) {
        // dispatches compute passes, fields stay separate buffers
        todo!()
    }

    fn snapshot(&self) -> &ParticleSnapshot {
        // GPU->CPU readback into self.readback, per-field
        todo!()
    }

    fn particle_count(&self) -> usize {
        todo!()
    }
}
