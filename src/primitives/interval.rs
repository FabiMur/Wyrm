use crate::utils::{INFINITY, NEG_INFINITY};

#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    // Main constructor
    pub fn new(min: f64, max: f64) -> Self {
        Interval {
            min: min,
            max: max,
        }
    }

    // Constructor from 2 intervals, encloses both
    pub fn new_from_aabboxs(a: &Self, b: &Self) -> Self {
        let min = if a.min <= b.min {a.min} else {b.min};
        let max = if a.max >= b.max {a.max} else {b.max};

        Self { min, max }
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

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else{
            x
        }
    }

    // Pad an interval with a certain delta
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta/2.0;
        Interval { 
            min: self.min - padding,
            max: self.max + padding
        }
    }
    
}

