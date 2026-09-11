// SphPlugin and Plugin implementation.

use std::num::NonZeroU32;
use std::sync::Arc;

use nih_plug::prelude::*;
use sph_core::ParticleSnapshot;
use sph_synth::SphSynth;

use crate::params::SphPluginParams;

pub(crate) struct SphPlugin {
    params: Arc<SphPluginParams>,
    sim_snapshot_rx: rtrb::Consumer<ParticleSnapshot>,
    synth: SphSynth,
}

impl Default for SphPlugin {
    fn default() -> Self {
        todo!()
    }
}

impl Plugin for SphPlugin {
    const NAME: &'static str = "SPH Synth";
    const VENDOR: &'static str = "Aseem Ratha";
    const URL: &'static str = "https://example.com";
    const EMAIL: &'static str = "info@example.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        todo!()
    }
}

impl ClapPlugin for SphPlugin {
    const CLAP_ID: &'static str = "com.aseemratha.sph-synth";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("SPH fluid-driven synthesizer");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
    ];
}

impl Vst3Plugin for SphPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"SphSynthPlugin01";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}
