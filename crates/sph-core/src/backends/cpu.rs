// CpuBackend.

use crate::{ParticleSet, ParticleSnapshot, SphBackend, SphParams, SpatialHash};

pub struct CpuBackend {
    particles: ParticleSet,
    hash: SpatialHash,
}

impl CpuBackend {
    pub fn new(initial: ParticleSet) -> Self {
        todo!()
    }
}

impl SphBackend for CpuBackend {
    fn step(&mut self, dt: f32, params: &SphParams) {
        todo!()
    }

    fn snapshot(&self) -> &ParticleSnapshot {
        todo!()
    }

    fn particle_count(&self) -> usize {
        todo!()
    }
}
