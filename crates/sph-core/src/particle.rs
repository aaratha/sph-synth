// ParticleSet, ParticleView, and ParticleSnapshot alias.

use crate::math::Vec3;

// Core SoA container — canonical representation used everywhere
pub struct ParticleSet {
    pub positions: Vec<Vec3>,
    pub velocities: Vec<Vec3>,
    pub densities: Vec<f32>,
    pub pressures: Vec<f32>,
    pub masses: Vec<f32>,
}

impl ParticleSet {
    pub fn with_capacity(n: usize) -> Self {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn particle(&self, i: usize) -> ParticleView<'_> {
        todo!()
    }

    pub fn push(&mut self, pos: Vec3, vel: Vec3, mass: f32) {
        todo!()
    }

    pub fn swap_remove(&mut self, i: usize) {
        todo!()
    }
}

pub struct ParticleView<'a> {
    pub position: &'a Vec3,
    pub velocity: &'a Vec3,
    pub density: &'a f32,
    pub pressure: &'a f32,
}

// Snapshot is just ParticleSet — no separate type, no AoS/SoA conversion at the boundary
pub type ParticleSnapshot = ParticleSet;
