// MappingCurve implementations for parameter-to-parameter curves.

pub enum CurveShape {
    Linear,
    Exponential(f32),
}

pub struct MappingCurve {
    pub input_range: (f32, f32),
    pub output_range: (f32, f32),
    pub shape: CurveShape,
}

impl MappingCurve {
    pub fn map(&self, value: f32) -> f32 {
        todo!()
    }
}
