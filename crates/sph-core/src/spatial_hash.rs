// SpatialHash.

use crate::math::Vec3;

// Index-based neighbor search — layout-agnostic by design
pub struct SpatialHash { /* ... */ }
impl SpatialHash {
    pub fn new(cell_size: f32) -> Self {
        todo!()
    }

    pub fn rebuild(&mut self, positions: &[Vec3]) {
        todo!()
    }

    pub fn query_neighbors(&self, pos: Vec3, radius: f32) -> Vec<usize> {
        todo!()
    }
}
