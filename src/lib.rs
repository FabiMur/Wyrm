// src/lib.rs
pub mod camera;
pub mod hittable;
pub mod materials;
pub mod primitives;
pub mod utils;

pub use materials::*;
pub use primitives::*;
pub use hittable::*;
pub use camera::*;