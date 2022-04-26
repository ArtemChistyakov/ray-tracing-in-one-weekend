use std::fs::File;
use std::io::Write;
use std::rc::Rc;

use ray_tracing_in_one_weekend::camera::Camera;
use ray_tracing_in_one_weekend::color;
use ray_tracing_in_one_weekend::color::Color;
use ray_tracing_in_one_weekend::material::{Dielectric, Lambertian, Metal, Scatter};
use ray_tracing_in_one_weekend::raytracing::{HittableList, random_double, random_double_range, ray_color, Sphere};
use ray_tracing_in_one_weekend::vec::{Point3, Vec3};

fn main() {
    let path = "images/image_19.ppm";
    let mut f = File::create(path).unwrap();

    //image
    let image_width = 1200;
    let aspect_ratio = 3.0 / 2.0;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let world = random_scene();

    //camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(look_from, look_at, vup, 20.0, aspect_ratio, aperture,
                             dist_to_focus);

    //render
    writeln!(f, "P3\n{} {}\n255", image_width, image_height).unwrap();

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
            }
            color::write_color(&mut f, &pixel_color, samples_per_pixel);
        }
    }
}

pub fn random_scene() -> HittableList {
    // World
    let mut world: HittableList = HittableList::new();
    let material_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, material_ground.clone())));

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1.clone())));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2.clone())));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3.clone())));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Scatter>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    world
}