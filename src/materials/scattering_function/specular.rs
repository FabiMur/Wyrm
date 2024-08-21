use std::sync::Arc;
use crate::primitives::*;
use crate::hittable::HitRecord;
use super::scattering_function::ScatteringFunction;

#[derive(Default)]
pub struct Specular;

impl Specular {
    pub fn new() -> Arc<dyn ScatteringFunction> {
        Arc::new(Specular) as Arc<dyn ScatteringFunction>
    }
}

impl ScatteringFunction for Specular {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);  // White specular reflection
        let unit_direction = r_in.dir.unit_vector();
        let direction = reflect(&unit_direction, &rec.normal);
        *scattered = Ray::new(rec.p, direction);
        true
    }
}
