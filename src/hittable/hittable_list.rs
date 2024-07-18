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
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, &mut Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
    
    fn bounding_box(&self) -> AABBox {
        self.bbox
    }
}
