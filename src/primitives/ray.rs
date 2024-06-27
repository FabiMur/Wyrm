use crate::primitives::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    // Public constructor to create a new instance of the ray
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    // Public method to obtain de origin point of the ray
    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    // Public method to obtain the direction of the ray
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    // Public method to calculate a point t distance in de direction of the ray
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + t * self.dir
    }
}

