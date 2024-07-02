use std::sync::Arc;
use crate::primitives::*;
use crate::hittable::HitRecord;
use crate::materials::{Material, MaterialArcWrapper};

// Diffuse Materials using Lambert's Cosine Law
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> MaterialArcWrapper {
        MaterialArcWrapper(Arc::new(Lambertian { albedo }) as Arc<dyn Material>)
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray {
            orig: rec.p,
            dir: scatter_direction,
        };
        *attenuation = self.albedo;
        true
    }
}
