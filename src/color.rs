use std::io::{self, Write};
use crate::vec3::Vec3;
use crate::interval::Interval;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> io::Result<()> {
    let r = pixel_color.x;
    let g = pixel_color.y;
    let b = pixel_color.z;

    // Translate the color components from the range [0,1] to [0,255].
    let intensity: Interval = Interval::new(0.000, 0.999);
    let rbyte = 256.0 * intensity.clamp(r);
    let gbyte = 256.0 * intensity.clamp(g);
    let bbyte = 256.0 * intensity.clamp(b);

    // Escribir los componentes del color del píxel.
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}
