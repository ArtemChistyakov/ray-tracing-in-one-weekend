use std::io::Write;

use crate::Vec3;

pub type Color = Vec3;

pub fn write_color<T: Write>(out: &mut T, pixel_color: &Color) {
    writeln!(out, "{} {} {}", (255.999 * pixel_color.x) as i32,
             (255.999 * pixel_color.y) as i32,
             (255.999 * pixel_color.z) as i32).unwrap();
}