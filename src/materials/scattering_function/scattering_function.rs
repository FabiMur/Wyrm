use crate::primitives::{Ray, Color};
use crate::hittable::HitRecord;

pub trait ScatteringFunction: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}
