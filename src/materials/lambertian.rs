use std::sync::Arc;
use crate::primitives::*;
use crate::hittable::HitRecord;
use crate::materials::{Material, MaterialArcWrapper};
use crate::textures::{Texture,SolidColor};

// Diffuse Materials using Lambert's Cosine Law
pub struct Lambertian {
    pub texture: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> MaterialArcWrapper {
        let solid_color_texture = Arc::new(SolidColor::new(albedo)) as Arc<dyn Texture>;
        MaterialArcWrapper(Arc::new(Lambertian { texture: solid_color_texture }) as Arc<dyn Material>)
    }

    pub fn new_from_texture(texture: Arc<dyn Texture>) -> MaterialArcWrapper {
        MaterialArcWrapper(Arc::new(Lambertian { texture }) as Arc<dyn Material>)
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
        *attenuation = self.texture.value(rec.u, rec.v, &rec.p);
        true
    }
}
