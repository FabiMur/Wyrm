use std::sync::Arc;

use crate::materials::Material;
use crate::hittable::{Hittable, HitRecord};
use crate::primitives::*;
use crate::bvh::AABBox;

const EPSILON: f64 = 0.00001;
pub struct Triangle {
    pub corner: Point3,       // One of the corners of the quad
    pub edge1: Vec3,          // Vector representing one edge of the quad
    pub edge2: Vec3,          // Vector representing the other edge of the quad
    pub mat: Arc<Material>,
    pub bbox: AABBox,         // Axis-aligned bounding box
    pub normal: Vec3,         // Surface normal
    pub offset: f64,          // Offset from the origin along the normal
}

impl Triangle {
    pub fn new(corner: Point3, edge1: Vec3, edge2: Vec3, mat: Arc<Material>) -> Self {
        let bbox_diagonal1 = AABBox::new_from_points(&corner, &(corner + edge1 + edge2));
        let bbox_diagonal2 = AABBox::new_from_points(&(corner + edge1), &(corner + edge2));
        let bbox = AABBox::new_from_aabboxs(&bbox_diagonal1, &bbox_diagonal2);

        let normal = edge1.cross(&edge2).unit_vector();
        let offset = normal.dot(&corner);

        Triangle {
            corner,
            edge1,
            edge2,
            mat,
            bbox,
            normal,
            offset,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
        let denom = self.normal.dot(&r.dir);

        // If the ray is parallel to the triangle's plane, there's no hit
        if f64::abs(denom) < EPSILON {
            return None;
        }

        // Calculate parameter t for the ray intersection point
        let t = (self.offset - self.normal.dot(&r.orig)) / denom;

        // Check if t is within the ray interval
        if !ray_t.contains(t) {
            return None;
        }

        // Calculate the intersection point
        let intersection = r.at(t);

        // Calculate coordinates in the triangle's local space
        let local_coords = intersection - self.corner;
        let alpha = self.edge2.cross(&local_coords).dot(&self.normal) / self.edge2.cross(&self.edge1).dot(&self.normal);
        let beta = local_coords.cross(&self.edge1).dot(&self.normal) / self.edge2.cross(&self.edge1).dot(&self.normal);

        // Check if the intersection point is inside the triangle
        let mut u = 0.0;
        let mut v = 0.0;

        if !is_interior_triangle(alpha, beta, &mut u, &mut v) {
            return None;
        }
       // Hit Record Initialization
       let mut rec: HitRecord = HitRecord::new(intersection, self.normal, self.mat.clone(), t, u, v, false);
       rec.set_face_normal(r, self.normal);

       Some(rec)
    }

    fn bounding_box(&self) -> AABBox {
        self.bbox
    }
}

fn is_interior_triangle(a: f64, b: f64, u: &mut f64, v: &mut f64) -> bool{
    if a < 0.0 || b < 0.0 || a + b > 1.0 {
        return false;
    }

    *u = a;
    *v = b;
    true
}
