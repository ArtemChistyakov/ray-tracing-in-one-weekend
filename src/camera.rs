use crate::{vec, Vec3};
use crate::raytracing::{degrees_to_radians, Ray};
use crate::vec::Point3;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(look_from: Point3,
               look_at: Point3,
               vup: Vec3,
               vfov: f64,
               aspect_ratio: f64) -> Camera { // vfow vertical field-of-view in degrees
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vec::unit_vector(look_from - look_at);
        let u = vec::unit_vector(vec::cross(&vup, &w));
        let v = vec::cross(&w, &u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}
