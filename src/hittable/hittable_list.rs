use std::sync::Arc;
use crate::hittable::{Hittable, HitRecord};
use crate::primitives::*;
use crate::bvh::AABBox;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    pub bbox: AABBox,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: Vec::new(), bbox: AABBox::new(Interval::empty(), Interval::universe(), Interval::universe())}
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.bbox = AABBox::new_from_aabboxs(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut rec: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, &mut Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = temp_rec.t;   
                rec = Some(temp_rec.clone());
            }
        }

        rec
    }
    
    fn bounding_box(&self) -> AABBox {
        self.bbox
    }
}
