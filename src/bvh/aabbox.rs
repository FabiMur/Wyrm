use crate::primitives::*;

#[derive(Clone, Copy, Debug)]
pub struct AABBox {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABBox {
    // Main constructor
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut aabox = AABBox { x, y, z };
        aabox.pad_to_minimums();
        aabox
    }

    // Construct the bounding box taking 2 points as the extremes of the parallelepiped
    pub fn new_from_points(a: &Point3, b: &Point3) -> Self {
        let (x_min, x_max) = if a.x <= b.x { (a.x, b.x) } else { (b.x, a.x) };
        let (y_min, y_max) = if a.y <= b.y { (a.y, b.y) } else { (b.y, a.y) };
        let (z_min, z_max) = if a.z <= b.z { (a.z, b.z) } else { (b.z, a.z) };

        let x_interval = Interval { min: x_min, max: x_max };
        let y_interval = Interval { min: y_min, max: y_max };
        let z_interval = Interval { min: z_min, max: z_max };

       AABBox::new(x_interval, y_interval, z_interval)
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

    // Getter function. 
    pub fn axis_interval(&self, n: usize) -> Interval {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid axis index"),
        }
    }

    // Longest dimension of the parallelepiped
    pub fn longest_axis(&self) -> usize {
        let x_length = self.x.max - self.x.min;
        let y_length = self.y.max - self.y.min;
        let z_length = self.z.max - self.z.min;

        if x_length > y_length && x_length > z_length {
            0
        } else if y_length > z_length {
            1
        } else {
            2
        }
    }

    // As when calculating the intersection between the bboxes we dont need the surface normal, hit point or
    // any other stuff, this method is similar but no not the one of the hittable trait.
    pub fn hit(&self, r: &Ray, ray_t: &mut Interval) -> bool {

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

            if ray_t.max.min(t1) <= ray_t.min.max(t0) {
                return false;
            }
        }

        true
    }

    fn pad_to_minimums(&mut self) -> () {
        let delta: f64 = 0.0001;
        
        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }

        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }

        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }
}
