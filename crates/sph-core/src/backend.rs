// SphBackend trait.

use crate::{ParticleSnapshot, SphParams};

// Implemented by CpuBackend (here) and GpuBackend (in sph-core-gpu)
pub trait SphBackend {
    fn step(&mut self, dt: f32, params: &SphParams);
    fn snapshot(&self) -> &ParticleSnapshot;
    fn particle_count(&self) -> usize;
}
