// SphSynth and render().

use sph_core::ParticleSnapshot;

use crate::params::SynthParams;

pub struct SphSynth {
    params: SynthParams,
    // internal oscillator/filter/grain state, persists across render() calls
}

impl SphSynth {
    pub fn new(params: SynthParams, sample_rate: f32) -> Self {
        todo!()
    }

    // Iterates snapshot.densities / .velocities / .pressures directly as flat slices —
    // no per-particle struct indirection on the hot path
    pub fn render(&mut self, snapshot: &ParticleSnapshot, out: &mut [f32]) {
        todo!()
    }

    pub fn set_params(&mut self, params: SynthParams) {
        todo!()
    }
}
