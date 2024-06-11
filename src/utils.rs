use std::f64::consts::PI;
use std::sync::Arc;

// Constants

pub const INFINITY: f64 = f64::INFINITY;
pub const PI_VALUE: f64 = PI;

// Utility functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// Alias for common types

pub type Shared<T> = Arc<T>;
