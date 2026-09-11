// render_offline: headless WAV rendering.

use sph_core::{SphBackend, SphSim};
use sph_synth::SphSynth;

fn render_offline<B: SphBackend>(
    sim: &mut SphSim<B>,
    synth: &mut SphSynth,
    duration_secs: f32,
) -> Vec<f32> {
    todo!()
}
