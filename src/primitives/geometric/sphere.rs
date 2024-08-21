use std::f64::consts::PI;
use std::sync::Arc;

use crate::hittable::{Hittable, HitRecord};
use crate::materials::Material;
use crate::primitives::*;
use crate::bvh::AABBox;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Arc<Material>,
    pub bbox: AABBox
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<Material>) -> Self {
        let rvec = Vec3 { x: radius, y: radius, z: radius};
        let bbox = AABBox::new_from_points(&(center - rvec), &(center + rvec));
        Sphere { center, radius, mat , bbox}
    }

    pub fn get_uv(p: &Point3) -> (f64, f64){
        let theta: f64 = f64::acos(-p.y);
        let phi: f64 = f64::atan2(-p.z, p.x) + PI;

        let u: f64 = phi / (2.0 * PI);
        let v: f64 = theta / PI;

        (u , v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
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
                let outward_normal = (r.at(root) - self.center) / self.radius;
                let u: f64;
                let v: f64;
                (u, v) = Sphere::get_uv(&outward_normal);

                let mut rec = HitRecord::new(
                    r.at(root),
                    outward_normal,
                    self.mat.clone(),
                    root,
                    u,
                    v,
                    false
                );
                rec.set_face_normal(r, outward_normal);

                return Some(rec);
            }

            root = (-half_b + sqrtd) / a;
            if root < ray_t.max && root > ray_t.min {
                let outward_normal = (r.at(root) - self.center) / self.radius;
                let u: f64;
                let v: f64;
                (u, v) = Sphere::get_uv(&outward_normal);

                let mut rec = HitRecord::new(
                    r.at(root),
                    outward_normal,
                    self.mat.clone(),
                    root,
                    u,
                    v,
                    false
                );
                rec.set_face_normal(r, outward_normal);

                return Some(rec);
            }
        }

        None
    }

    fn bounding_box(&self) -> AABBox {
        self.bbox
    }
}
