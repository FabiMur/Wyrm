use std::sync::Arc;
use crate::hittable::{Hittable, HitRecord};
use crate::primitives::*;
use crate::bvh::AABBox;
use std::cmp::Ordering;

pub struct BVHNode {
    pub bbox: AABBox,
    pub left: Arc<dyn Hittable + Send + Sync>,
    pub right: Arc<dyn Hittable + Send + Sync>,
}

impl BVHNode {
    pub fn new(mut objects: Vec<Arc<dyn Hittable + Send + Sync>>, start: usize, end: usize) -> Self {
        let mut bbox = AABBox::new_empty();

        // Create a new bbox that enclouures all the objects in the list. 
        for object_index in start..end {
            bbox = AABBox::new_from_aabboxs(&bbox, &objects[object_index].bounding_box());
        }

        // Get the longest axis of the resulting bbox along which we will divide into subboxes
        let axis = bbox.longest_axis();
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_count: usize = end - start;

        let (left, right) = if object_count == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_count == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                (objects[start].clone(), objects[start + 1].clone())
            } else {
                (objects[start + 1].clone(), objects[start].clone())
            }
        } else {
            objects[start..end].sort_by(|a, b| comparator(a, b));
            let mid = start + object_count / 2;
            let left = Arc::new(BVHNode::new(objects.clone(), start, mid)) as Arc<dyn Hittable + Send + Sync>;
            let right = Arc::new(BVHNode::new(objects, mid, end)) as Arc<dyn Hittable + Send + Sync>;
            (left, right)
        };

        BVHNode { bbox, left, right }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        // If the ray doesn't hit the bbox just return false
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let mut hit_anything = false;

        // If the ray hits left node update ray_t and rec so that it aslso hits 
        // the right nod it has to be in a t closer the origin the one on left.
        if self.left.hit(r, ray_t, rec) {
            hit_anything = true;
            *ray_t = Interval { min: ray_t.min, max: rec.t };
        }

        // If the ray hits right node
        if self.right.hit(r, ray_t, rec) {
            hit_anything = true;
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABBox {
        self.bbox
    }
}

fn box_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>, axis_index: i32) -> Ordering {
    let a_axis_interval = a.bounding_box().axis_interval(axis_index as usize);
    let b_axis_interval = b.bounding_box().axis_interval(axis_index as usize);

    a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap()
}

fn box_x_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable + Send + Sync>, b: &Arc<dyn Hittable + Send + Sync>) -> Ordering {
    box_compare(a, b, 2)
}
