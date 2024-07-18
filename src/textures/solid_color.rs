use crate::primitives::*;
use crate::textures::Texture;

pub struct SolidColor {
    pub albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u:f64, _v:f64, _p: &Point3) -> Color {
        self.albedo
    }
}
