// SphPluginParams.

use nih_plug::prelude::*;

#[derive(Params)]
pub struct SphPluginParams {
    #[id = "viscosity"]
    pub viscosity: FloatParam,
    #[id = "rest_density"]
    pub rest_density: FloatParam,
}

impl Default for SphPluginParams {
    fn default() -> Self {
        Self {
            viscosity: FloatParam::new("Viscosity", 0.1, FloatRange::Linear { min: 0.0, max: 1.0 }),
            rest_density: FloatParam::new(
                "Rest Density",
                1000.0,
                FloatRange::Linear { min: 100.0, max: 2000.0 },
            ),
        }
    }
}
