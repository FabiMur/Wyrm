use std::io::{self, Write};
use crate::vec3::Vec3;
pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> io::Result<()> {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Traducir los valores del componente [0,1] al rango de bytes [0,255].
    let rbyte = (255.999 * r) as u8;
    let gbyte = (255.999 * g) as u8;
    let bbyte = (255.999 * b) as u8;

    // Escribir los componentes del color del píxel.
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte)
}
