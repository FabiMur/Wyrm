use std::sync::Arc;
use crate::primitives::*;
use crate::textures::*;

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self { inv_scale: 1.0/scale, even, odd }
    }

    pub fn new_from_colors(scale: f64, even: Color, odd: Color) -> Self {
        let even_texture: SolidColor = SolidColor::new(even);
        let odd_texture: SolidColor = SolidColor::new(odd);
        Self { inv_scale: 1.0/scale, even:Arc::new(even_texture), odd:Arc::new(odd_texture) }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u:f64, v:f64, p: &Point3) -> Color {
        let x_integer: i32 = (self.inv_scale * p.x) as i32;
        let y_integer: i32 = (self.inv_scale * p.y) as i32;
        let z_integer: i32 = (self.inv_scale * p.z) as i32;

        let is_even: bool = (x_integer + y_integer + z_integer) % 2 == 0;

        if is_even {
            return self.even.value(u, v, p);
        } else {
            return self.odd.value(u, v, p);
        }

    }
}

impl Default for CheckerTexture {
    fn default() -> Self {
        CheckerTexture {
            inv_scale: 1.0/64.0,
            even: Arc::new(SolidColor::new(Color::new(1.0,0.0, 0.0))),
            odd: Arc::new(SolidColor::new(Color::new(0.0,0.0, 1.0))),
        }
    }
}