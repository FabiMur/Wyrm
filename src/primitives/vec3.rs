use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};
use crate::utils::{random_double_range};

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    //fn random() -> Self {
    //    Self{x: random_double(), y: random_double(), z: random_double()}
    //}

    // Generates a random vector with componentes initialized between min and max.
    pub fn random_in_range(min: f64, max: f64) -> Self {
        Self{x: random_double_range(min, max), y: random_double_range(min, max), z: random_double_range(min, max)}
    }

    // Generates a vector with a modulus under 1
    pub fn random_in_unit_sphere() -> Self {
        let mut p: Vec3 = Vec3::random_in_range(-1.0, 1.0);
        loop {
            if p.length_squared() < 1.0 {
                break;
            }
            p = Vec3::random_in_range(-1.0, 1.0);
        }

        return p;
    }

    pub fn random_unit_vector() -> Self {
        return unit_vector(&Vec3::random_in_unit_sphere())
    }

    // Generates a random vector in the same hemisphere of another vector
    // (The surface normal of an impact)
    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Vec3::random_unit_vector();
        if dot(&on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn near_zero(&self) -> bool {
        let s: f64 = 1e-8;
        self[0].abs() < s &&  self[0].abs() < s && self[2].abs() < s
    }

}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3 {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds"),
        }
    }
}

// Reflects a vector 'v' around a normal vector 'n' using the 
// reflection formula: r = v - 2 * (dot(v, n) * n).
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - (2.0*dot(v, n) * *n)
}

// Computes the refraction of a vector 'uv' through a surface with normal 'n'
// according to Snell's law.
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let neg_uv: Vec3 = - *uv;
    let cos_theta = f64::min(dot(&neg_uv, n), 1.0);
    let r_out_perp: Vec3 = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * *n;
    
    r_out_perp + r_out_parallel
}

// Wrapper for dot method
#[inline(always)]
pub fn dot(v: &Vec3, w: &Vec3) -> f64 {
    v.dot(w)
}

// Wrapper for unit_vector method
#[inline(always)]
pub fn unit_vector(v: &Vec3) -> Vec3 {
    v.unit_vector()
}

pub type Point3 = Vec3;
