use std::f64::consts::PI;
use std::sync::Arc;
use fastrand;

// Constants

pub const INFINITY: f64 = f64::INFINITY;
pub const NEG_INFINITY: f64 = f64::NEG_INFINITY;
pub const PI_VALUE: f64 = PI;

// Utility functions
#[inline(always)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Returns a random real in [0, 1).
#[inline(always)]
pub fn random_double() -> f64 {
    fastrand::f64()
}

/// Returns a random real in [min, max).
#[inline(always)]
pub fn random_double_range(min: f64, max: f64) -> f64 {
    fastrand::f64() * (max - min) + min
}

// Alias for common types

pub type Shared<T> = Arc<T>;
