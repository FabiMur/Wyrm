use std::sync::Arc;
use crate::hittable::{Hittable, HitRecord};
use crate::primitives::*;
use crate::bvh::AABBox;
use crate::utils::{degrees_to_radians, INFINITY, NEG_INFINITY};


pub struct RotationY {
    pub object: Arc<dyn Hittable + Send + Sync>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: AABBox
}

impl RotationY {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta: f64 = f64::sin(radians);
        let cos_theta: f64 = f64::cos(radians);
        
        let mut bbox: AABBox = object.bounding_box();

        let mut min: Point3 = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max: Point3 = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - i as f64)*bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64)*bbox.y.min;
                    let z = k as f64 *bbox.z.max + (1.0 - k as f64)*bbox.z.min;

                    let newx = cos_theta*x + sin_theta*z;
                    let newz = -sin_theta*x + cos_theta*z;

                    let tester: Vec3 = Vec3::new(newx, y, newz);

                    for l in 0..2 {
                        min[l] = f64::min(min[l], tester[l]);
                        max[l] = f64::max(max[l], tester[l]);
                    }

                }
            }
        }

        bbox = AABBox::new_from_points(&min, &max);

        Self {object, sin_theta, cos_theta, bbox}
    }
}

impl Hittable for RotationY {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        // change the ray from world space to object space
        let mut origin: Point3 = r.origin();
        let mut direction: Vec3 = r.direction();

        origin[0] = self.cos_theta* r.origin()[0] - self.sin_theta*r.origin()[2];
        origin[2] = self.sin_theta* r.origin()[0] + self.cos_theta*r.origin()[2];

        direction[0] = self.cos_theta* r.direction()[0] - self.sin_theta*r.direction()[2];
        direction[2] = self.sin_theta* r.direction()[0] + self.cos_theta*r.direction()[2];

        let rotated_r: Ray = Ray::new(origin, direction);

        // Determine whether an intersection exists in object space (and if so, where)
        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false
        }

        // Change the intersection point from object space to world space
        let mut p:Point3 = rec.p;
        p[0] = self.cos_theta*rec.p[0] + self.sin_theta*rec.p[2];
        p[2] = -self.sin_theta*rec.p[0] + self.cos_theta*rec.p[2];

        // Change the object normal from object space to world space
        let mut normal: Point3 = rec.normal;
        normal[0] = self.cos_theta*rec.normal[0] + self.sin_theta*rec.normal[2];
        normal[2] = -self.sin_theta*rec.normal[0] + self.cos_theta*rec.normal[2];

        rec.p = p;
        rec.normal = normal;

        return true;
    }

    fn bounding_box(&self) -> AABBox {
        self.bbox
    }
}