use std::fs::File;
use std::io::{ErrorKind, Write};

use ray_tracing_in_one_weekend::Vec3;

fn main() {
    let path = "chapter_1_image.ppm";
    let mut f = File::create(path).unwrap();
    let nx = 200;
    let ny = 100;
    writeln!(f, "P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let vec3 = Vec3::new(i as f64 / nx as f64,
                                 j as f64 / ny as f64, 0.2);
            let ir = (255.99 * vec3.x) as i32;
            let ig = (255.99 * vec3.y) as i32;
            let ib = (255.99 * vec3.z) as i32;
            writeln!(f, "{} {} {}", ir, ig, ib);
        }
    }
}
