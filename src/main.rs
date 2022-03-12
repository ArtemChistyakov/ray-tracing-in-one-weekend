use std::fs::File;
use std::io::Write;
use std::rc::Rc;

use ray_tracing_in_one_weekend::{color};
use ray_tracing_in_one_weekend::camera::Camera;
use ray_tracing_in_one_weekend::color::Color;
use ray_tracing_in_one_weekend::raytracing::{HittableList, random_double, ray_color, Sphere};
use ray_tracing_in_one_weekend::vec::{Point3};

fn main() {
    let path = "images/chapter_8.5_image.ppm";
    let mut f = File::create(path).unwrap();

    //image
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world: HittableList = HittableList::with_capacity(2);
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));

    let camera = Camera::new();

    //render
    writeln!(f, "P3\n{} {}\n255", image_width, image_height).unwrap();

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world,max_depth);
            }
            color::write_color(&mut f, &pixel_color, samples_per_pixel);
        }
    }
}
