use std::sync::Arc;
use crate::primitives::*;
use crate::hittable::HitRecord;
use crate::materials::{Material, MaterialArcWrapper};
use crate::utils::random_double;

pub struct Dielectric {
    refraction_index: f64,
}

// Dielectric Materials, sometimes specular and sometimes refractive.
// Following the Law of Reflection or Snell's Law
impl Dielectric {
    pub fn new(refraction_index: f64) -> MaterialArcWrapper {
        MaterialArcWrapper(Arc::new(Dielectric { refraction_index }) as Arc<dyn Material>)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri: f64;
        if rec.front_face {
            ri = 1.0 / self.refraction_index;
        } else {
            ri = self.refraction_index;
        }

        let unit_direction: Vec3 = r_in.dir.unit_vector();
        let neg_unit_direction: Vec3 = -unit_direction;
        let cos_theta: f64 = f64::min(dot(&neg_unit_direction, &rec.normal), 1.0);
        let sin_theta: f64 = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract: bool = (ri * sin_theta) > 1.0;
        let direction: Vec3;

        if cannot_refract || reflectance(cos_theta, ri) > random_double() {
            // Specular behavior
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            // Refractive behavior
            direction = refract(&unit_direction, &rec.normal, ri);
        }

        *scattered = Ray::new(rec.p, direction);
        true
    }
}


// Schlick Approximation for Fresnel reflectance
fn reflectance(cosine: f64, refraction_index: f64) -> f64{
    let mut r0: f64 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5);
}