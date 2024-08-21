use std::sync::Arc;
use crate::primitives::*;
use crate::hittable::HitRecord;
use crate::textures::{Texture,SolidColor};

use super::ScatteringFunction;

// Diffuse Materials using Lambert's Cosine Law
pub struct Lambertian {
    texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Arc<dyn ScatteringFunction> {
        let solid_color_texture = Arc::new(SolidColor::new(albedo)) as Arc<dyn Texture>;
        Arc::new(Lambertian { texture: solid_color_texture }) as Arc<dyn ScatteringFunction>
    }

    pub fn new_from_texture(texture: Arc<dyn Texture>) -> Arc<dyn ScatteringFunction> {
        Arc::new(Lambertian { texture }) as Arc<dyn ScatteringFunction>
    }
}

impl ScatteringFunction for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray {
            orig: rec.p,
            dir: scatter_direction,
        };
        *attenuation = self.texture.value(rec.u, rec.v, &rec.p);
        true
    }
}

impl Default for Lambertian {
    fn default() -> Self {
        Self {
            texture: Arc::new(SolidColor::default())
        }
    }
}
