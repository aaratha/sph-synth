// SphSim<B>.

use crate::{ParticleSnapshot, SphBackend, SphParams};

pub struct SphSim<B: SphBackend> {
    backend: B,
    params: SphParams,
}

impl<B: SphBackend> SphSim<B> {
    pub fn new(backend: B, params: SphParams) -> Self {
        todo!()
    }

    pub fn step(&mut self, dt: f32) {
        todo!()
    }

    pub fn snapshot(&self) -> &ParticleSnapshot {
        todo!()
    }
}
