// SphParams.

use crate::math::Vec3;

pub struct SphParams {
    pub h: f32,
    pub rest_density: f32,
    pub stiffness: f32,
    pub viscosity: f32,
    pub gravity: Vec3,
}
