use std::io::{self, Write};
use crate::primitives::vec3::Vec3;
use crate::primitives::interval::Interval;

pub type Color = Vec3;

#[inline(always)]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt()
    }

    return 0.0
}

#[inline(always)]
pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> io::Result<()> {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Apply a linear to gamma transform for gamma 2
    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // Translate the color components from the range [0,1] to [0,255].
    let intensity: Interval = Interval::new(0.000, 0.999);
    let rbyte = 256.0 * intensity.clamp(r);
    let gbyte = 256.0 * intensity.clamp(g);
    let bbyte = 256.0 * intensity.clamp(b);

    // Escribir los componentes del color del píxel.
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}
