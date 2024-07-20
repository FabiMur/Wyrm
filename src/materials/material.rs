use crate::primitives::{Ray, Color};
use crate::hittable::HitRecord;
use crate::Point3;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;

    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub struct DefaultMaterial;

impl Material for DefaultMaterial {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _attenuation: &mut Color, _scattered: &mut Ray) -> bool {
        false
    }
}

impl Default for DefaultMaterial {
    fn default() -> Self {
        Self {}
    }
}
