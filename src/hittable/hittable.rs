use std::sync::Arc;

use crate::primitives::*;
use crate::materials::Material;
use crate::bvh::AABBox;
#[derive(Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub mat: Arc<Material>,  // Use Option to allow uninitialized material
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        p: Vec3,
        normal: Vec3,
        mat: Arc<Material>,
        t: f64,
        u: f64,
        v: f64,
        front_face: bool,
    ) -> Self {
        HitRecord {
            p,
            normal,
            mat,
            t,
            u,
            v,
            front_face,
        }
    }
    
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
    fn hit(&self, r: &Ray, ray_t: &mut Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> AABBox;
}