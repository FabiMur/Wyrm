use std::sync::Arc;
use crate::primitives::*;
use crate::hittable::HitRecord;
use crate::utils::random_double;
use super::scattering_function::ScatteringFunction;

#[derive(Default)]
pub struct Refractive {
    refraction_index: f64,
}

impl Refractive {
    pub fn new(refraction_index: f64) -> Arc<dyn ScatteringFunction> {
        Arc::new(Refractive { refraction_index }) as Arc<dyn ScatteringFunction>
    }
}

impl ScatteringFunction for Refractive {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);  // No color attenuation for refraction
        let refraction_ratio = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        
        let unit_direction: Vec3 = r_in.dir.unit_vector();
        let cos_theta = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_double() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        *scattered = Ray::new(rec.p, direction);
        true
    }
}

// Schlick Approximation for Fresnel reflectance
fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0: f64 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}