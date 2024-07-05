use crate::primitives::*;

#[derive(Clone, Copy)]
pub struct AABBox {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABBox {
    // Main constructor
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        AABBox { x, y, z }
    }

    // Construct the bounding box taking 2 points as the extremes of the parallelepiped
    pub fn new_from_points(a: &Point3, b: &Point3) -> Self {
        let (x_min, x_max) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
        let (y_min, y_max) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
        let (z_min, z_max) = if a.z <= b.z { (a.z, b.z) } else { (b.z, a.z) };

        let x_interval = Interval { min: x_min, max: x_max };
        let y_interval = Interval { min: y_min, max: y_max };
        let z_interval = Interval { min: z_min, max: z_max };

        Self { x: x_interval, y: y_interval, z: z_interval }
    }

    // Construct the bounding box enclosing another 2
    pub fn new_from_aabboxs(a: &Self, b: &Self) -> Self {
        let x = Interval::new_from_aabboxs(&a.x, &b.x);
        let y = Interval::new_from_aabboxs(&a.y, &b.y);
        let z = Interval::new_from_aabboxs(&a.z, &b.z);

        Self { x, y, z }
    }

    // BBox with no size
    pub fn new_empty() -> Self {
        let x = Interval::empty();
        let y = Interval::empty();
        let z = Interval::empty();

        Self { x, y, z }
    }

    pub fn axis_interval(&self, n: usize) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid axis index"),
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {
        let mut tmin = ray_t.min;
        let mut tmax = ray_t.max;
    
        for axis in 0..3 {
            let inv_d = 1.0 / r.dir[axis];
            let (t0, t1) = if inv_d >= 0.0 {
                (
                    (self.axis_interval(axis).min - r.orig[axis]) * inv_d,
                    (self.axis_interval(axis).max - r.orig[axis]) * inv_d,
                )
            } else {
                (
                    (self.axis_interval(axis).max - r.orig[axis]) * inv_d,
                    (self.axis_interval(axis).min - r.orig[axis]) * inv_d,
                )
            };
    
            tmin = tmin.max(t0);
            tmax = tmax.min(t1);
    
            if tmax <= tmin {
                return false;
            }
        }
    
        ray_t.min = tmin;
        ray_t.max = tmax;
    
        true
    }
    
}
