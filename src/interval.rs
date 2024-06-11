use crate::utils::{INFINITY, NEG_INFINITY};

#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    // Default interval is empty
    pub fn new(min: f64, max: f64) -> Self {
        Interval {
            min: min,
            max: max,
        }
    }

    // Size of the interval
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    // Check if interval contains a value
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    // Check if interval surrounds a value
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    // Empty interval
    pub fn empty() -> Self {
        Interval {
            min: INFINITY,
            max: NEG_INFINITY,
        }
    }

    // Universe interval
    pub fn universe() -> Self {
        Interval {
            min: NEG_INFINITY,
            max: INFINITY,
        }
    }
}