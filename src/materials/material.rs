use std::sync::Arc;
use std::ops::Deref;
use crate::primitives::{Ray, Color};
use crate::hittable::HitRecord;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
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

#[derive(Clone)]
pub struct MaterialArcWrapper(pub Arc<dyn Material>);

impl Default for MaterialArcWrapper {
    fn default() -> Self {
        MaterialArcWrapper(Arc::new(DefaultMaterial))
    }
}

impl Deref for MaterialArcWrapper {
    type Target = Arc<dyn Material>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
