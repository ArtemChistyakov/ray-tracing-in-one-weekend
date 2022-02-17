use std::fs::File;
use std::io::Write;

use ray_tracing_in_one_weekend::Ray;
use ray_tracing_in_one_weekend::vec::Vec3;

fn main() {
    let path = "images/chapter_4_image.ppm";
    let mut f = File::create(path).unwrap();
    let nx = 200;
    let ny = 100;
    writeln!(f, "P3\n{} {}\n255", nx, ny).unwrap();

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let color = ray.color();
            let ir = (255.99 * color.x) as i32;
            let ig = (255.99 * color.y) as i32;
            let ib = (255.99 * color.z) as i32;
            writeln!(f, "{} {} {}", ir, ig, ib).unwrap();
        }
    }
}
