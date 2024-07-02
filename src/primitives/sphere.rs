use crate::hittable::{Hittable, HitRecord};
use crate::materials::MaterialArcWrapper;
use crate::primitives::*;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: MaterialArcWrapper, // Use MaterialArcWrapper directly
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: MaterialArcWrapper) -> Self {
        Sphere { center, radius, mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&r.direction(), &oc);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let sqrtd = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range.
            let mut root = (-half_b - sqrtd) / a;
            if root < ray_t.max && root > ray_t.min {
                rec.t = root;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat = Some(self.mat.clone());

                return true;
            }

            root = (-half_b + sqrtd) / a;
            if root < ray_t.max && root > ray_t.min {
                rec.t = root;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat = Some(self.mat.clone());

                return true;
            }
        }
        false
    }
}
