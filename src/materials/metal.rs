use std::sync::Arc;
use crate::primitives::*;
use crate::hittable::HitRecord;
use crate::materials::{Material, MaterialArcWrapper};
use crate::primitives::Vec3;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> MaterialArcWrapper {
        MaterialArcWrapper(Arc::new(Metal { albedo, fuzz }) as Arc<dyn Material>)
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut reflected: Vec3 = reflect(&r_in.direction().unit_vector(), &rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return Vec3::dot(&scattered.direction(), &rec.normal) > 0.0;
    }
}