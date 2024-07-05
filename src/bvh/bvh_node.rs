use std::sync::Arc;
use crate::hittable::{Hittable, HitRecord};
use crate::primitives::*;
use crate::bvh::AABBox;
use std::cmp::Ordering;

pub struct BVHNode {
    pub bbox: AABBox,
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
}

impl BVHNode {
    pub fn new(mut objects: Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self { 
        
        
        let mut bbox = AABBox::new_empty();
        for object_index in start..end {
            bbox = AABBox::new_from_aabboxs(&bbox,&objects[object_index].bounding_box());
        }
        
        let axis = bbox.longest_axis();
            
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_span: usize = end - start;

        let (left, right) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                (objects[start].clone(), objects[start + 1].clone())
            } else {
                (objects[start + 1].clone(), objects[start].clone())
            }
        } else {
            objects[start..end].sort_by(|a, b| comparator(a, b));
            let mid = start + object_span / 2;
            let left = Arc::new(BVHNode::new(objects.clone(), start, mid)) as Arc<dyn Hittable>;
            let right = Arc::new(BVHNode::new(objects, mid, end)) as Arc<dyn Hittable>;
            (left, right)
        };

        BVHNode { bbox, left, right }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        // Check if the ray hits the bounding box
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let mut hit_anything = false;

        // Check if the ray hits the left child
        if self.left.hit(r, ray_t, rec) {
            hit_anything = true;
            *ray_t = Interval { min: ray_t.min, max: rec.t };
        }

        // Check if the ray hits the right child with the updated interval
        if self.right.hit(r, ray_t, rec) {
            hit_anything = true;
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABBox {
        self.bbox
    }
}


fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: i32) -> Ordering {
    let a_axis_interval = a.bounding_box().axis_interval(axis_index as usize);
    let b_axis_interval = b.bounding_box().axis_interval(axis_index as usize);

    a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap()
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
