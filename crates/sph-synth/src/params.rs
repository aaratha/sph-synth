// SynthParams and MappingCurve.

use crate::mapping::MappingCurve;

pub struct SynthParams {
    pub density_to_amplitude: MappingCurve,
    pub velocity_to_pitch: MappingCurve,
    pub pressure_gradient_to_cutoff: MappingCurve,
}
