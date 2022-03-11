use std::io::Write;

use crate::Vec3;

pub type Color = Vec3;

pub fn write_color<T: Write>(out: &mut T, pixel_color: &Color, samples_per_pixel: usize) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

// Write the translated [0,255] value of each color component.
    writeln!(out, "{} {} {}", (256.0 * clamp(r, 0.0, 0.999)) as i32,
             (256.0 * clamp(g, 0.0, 0.999)) as i32,
             (256.0 * clamp(b, 0.0, 0.999)) as i32).unwrap();
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    x
}