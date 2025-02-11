use std::sync::Arc;
use crate::hittable::{Hittable, HitRecord};
use crate::primitives::*;
use crate::bvh::AABBox;


pub struct Translation {
    pub object: Arc<dyn Hittable + Send + Sync>,
    pub offset: Vec3,
    pub bbox: AABBox
}

impl Translation {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, offset: Vec3) -> Self {
        let bbox = object.bounding_box();
        Translation {object, offset, bbox}
    }
}

impl Hittable for Translation {
    fn hit(&self, r: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
        // Movre the ray backwards by the offset
        let offset_ray: Ray = Ray::new(r.origin() - self.offset, r.direction());

        // Determine whether an intersection exists along the offset ray (and if so, where)
        if let Some(mut rec) = self.object.hit(&offset_ray, ray_t) {
            rec.p = rec.p + self.offset;
            return Some(rec)
        } else {
            return None;
        }
    }

    fn bounding_box(&self) -> AABBox {
        self.bbox
    }
}