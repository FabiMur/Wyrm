use std::sync::Arc;

use crate::materials::Material;
use crate::hittable::{Hittable, HitRecord, HittableList};
use crate::primitives::*;
use crate::bvh::AABBox;

const EPSILON: f64 = 0.00001;

pub struct Quad {
    pub corner: Point3,       // One of the corners of the quad
    pub edge1: Vec3,          // Vector representing one edge of the quad
    pub edge2: Vec3,          // Vector representing the other edge of the quad
    pub mat: Arc<dyn Material>,
    pub bbox: AABBox,         // Axis-aligned bounding box
    pub normal: Vec3,         // Surface normal
    pub offset: f64,          // Offset from the origin along the normal
}

impl Quad {
    pub fn new(corner: Point3, edge1: Vec3, edge2: Vec3, mat: Arc<dyn Material>) -> Self {
        let bbox_diagonal1 = AABBox::new_from_points(&corner, &(corner + edge1 + edge2));
        let bbox_diagonal2 = AABBox::new_from_points(&(corner + edge1), &(corner + edge2));
        let bbox = AABBox::new_from_aabboxs(&bbox_diagonal1, &bbox_diagonal2);

        let normal = edge1.cross(&edge2).unit_vector();
        let offset = normal.dot(&corner);

        Quad {
            corner,
            edge1,
            edge2,
            mat,
            bbox,
            normal,
            offset,
        }
    }

    pub fn new_box(corner_a: Point3, corner_b: Point3, mat: Arc<dyn Material>) -> HittableList {
        // To store the 6 quads that form the 3D box
        let mut sides = HittableList::new();

        // Construct the 2 opposite vertices with the minimum and maximum cordinates
        let min: Point3 = Point3::new(f64::min(corner_a.x, corner_b.x), f64::min(corner_a.y, corner_b.y), f64::min(corner_a.z, corner_b.z));
        let max: Point3 = Point3::new(f64::max(corner_a.x, corner_b.x), f64::max(corner_a.y, corner_b.y), f64::max(corner_a.z, corner_b.z));

        let dx: Vec3 = Vec3::new(max.x - min.x, 0.0, 0.0);
        let dy: Vec3 = Vec3::new(0.0, max.y - min.y, 0.0);
        let dz: Vec3 = Vec3::new(0.0, 0.0, max.z - min.z);

        sides.add(Arc::new(Quad::new(Point3::new(min.x, min.y, max.z), dx, dy, mat.clone())));   // front
        sides.add(Arc::new(Quad::new(Point3::new(max.x, min.y, max.z), -dz, dy, mat.clone())));  // right
        sides.add(Arc::new(Quad::new(Point3::new(max.x, min.y, min.z), -dx, dy, mat.clone())));  // back
        sides.add(Arc::new(Quad::new(Point3::new(min.x, min.y, min.z), dz, dy, mat.clone())));   // left
        sides.add(Arc::new(Quad::new(Point3::new(min.x, max.y, max.z), dx, -dz, mat.clone())));  // top
        sides.add(Arc::new(Quad::new(Point3::new(min.x, min.y, max.z), dx, dz, mat.clone())));   // bottom

        sides
    }

}


impl Hittable for Quad { 
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal.dot(&r.dir);

        // If the ray is parallel to the quad's plane, there's no hit
        if f64::abs(denom) < EPSILON {
            return false;
        }

        // Calculate parameter t for the ray intersection point
        let t = (self.offset - self.normal.dot(&r.orig)) / denom;

        // Check if t is within the ray interval
        if !ray_t.contains(t) {
            return false;
        }

        // Calculate the intersection point
        let intersection = r.at(t);

        // Calculate coordinates in the parallelogram's local space
        let local_coords = intersection - self.corner;
        let alpha = self.edge2.cross(&local_coords).dot(&self.normal) / self.edge2.cross(&self.edge1).dot(&self.normal);
        let beta = local_coords.cross(&self.edge1).dot(&self.normal) / self.edge2.cross(&self.edge1).dot(&self.normal);

        // Check if the intersection point is inside the quad
        if !is_interior(alpha, beta, rec) {
            return false;
        }

        // Record the intersection details
        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> AABBox {
        self.bbox
    }
}

fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
    let unit_interval: Interval = Interval::new(0.0, 1.0);


    if !unit_interval.contains(a) || !unit_interval.contains(b) {
        return false;
    }

    rec.u = a;
    rec.v = b;
    true
}