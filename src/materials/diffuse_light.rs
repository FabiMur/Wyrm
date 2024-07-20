use std::sync::Arc;
use crate::primitives::*;
use crate::hittable::HitRecord;
use crate::materials::Material;
use crate::textures::{Texture,SolidColor};

pub struct DiffuseLight {
    texture: Arc<dyn Texture>
}

impl DiffuseLight {
    pub fn new(albedo: Color) -> Arc<dyn Material> {
        let solid_color_texture = Arc::new(SolidColor::new(albedo)) as Arc<dyn Texture>;
        Arc::new(DiffuseLight { texture: solid_color_texture }) as Arc<dyn Material>
    }

    pub fn new_from_texture(texture: Arc<dyn Texture>) -> Arc<dyn Material> {
        Arc::new(DiffuseLight { texture })
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _attenuation: &mut Color, _scattered: &mut Ray) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.texture.value(u, v, &p)
    }
}