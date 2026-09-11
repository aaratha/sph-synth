// Public API re-exports and crate-level documentation.

mod backend;
mod backends;
mod kernels;
mod math;
mod params;
mod particle;
mod sim;
mod spatial_hash;

pub use backend::SphBackend;
pub use backends::cpu::CpuBackend;
pub use kernels::{poly6, spiky_gradient, viscosity_laplacian};
pub use math::Vec3;
pub use params::SphParams;
pub use particle::{ParticleSet, ParticleSnapshot, ParticleView};
pub use sim::SphSim;
pub use spatial_hash::SpatialHash;
