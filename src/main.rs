use std::fs::File;
use std::io::Write;
use std::rc::Rc;

use ray_tracing_in_one_weekend::{color, raytracing};
use ray_tracing_in_one_weekend::raytracing::{HittableList, Ray, Sphere};
use ray_tracing_in_one_weekend::vec::{Point3, Vec3};

fn main() {
    let path = "images/chapter_6.7_image.ppm";
    let mut f = File::create(path).unwrap();

    //image
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // World
    let mut world: HittableList = HittableList::with_capacity(2);
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));


    //camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    //render
    writeln!(f, "P3\n{} {}\n255", image_width, image_height).unwrap();

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let dir = lower_left_corner + horizontal * u + vertical * v - origin;
            let r = Ray::new(origin, dir);
            let pixel_color = raytracing::ray_color(&r, &world);
            color::write_color(&mut f, &pixel_color);
        }
    }
}
