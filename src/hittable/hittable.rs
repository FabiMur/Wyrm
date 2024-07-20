use std::sync::Arc;

use crate::{primitives::*, Lambertian};
use crate::materials::Material;
use crate::bvh::AABBox;
#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,  // Use Option to allow uninitialized material
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = dot(&ray.dir, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> AABBox;
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::default(),
            normal: Vec3::default(),
            mat: Arc::new(Lambertian::default()),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false

        }
    }
}