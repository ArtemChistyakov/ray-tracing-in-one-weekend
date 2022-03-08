use crate::{vec, Vec3};
use crate::color::Color;
use crate::vec::Point3;

pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { orig: a, dir: b }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig.clone() + (self.dir.clone() * t)
    }
}

pub fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = r.origin().clone() - center.clone();
    let a = r.direction().length_squared();
    let half_b = vec::dot(&oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    return if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    };
}


pub fn ray_color(ray: &Ray) -> Color {
    let cen = Point3::new(0.0, 0.0, -1.0);
    let t = hit_sphere(&cen, 0.5, &ray);
    if t > 0.0 {
        let x = ray.at(t) - Point3::new(0.0, 0.0, -1.0);
        let n = vec::unit_vector(&x);
        return Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
    }
    let unit_direction = vec::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::raytracing::{calc_a, calc_b, calc_c, calc_discriminant, calc_oc, hit_sphere, Ray, ray_color};
    use crate::Vec3;
    use crate::vec::Point3;

    #[test]
    fn check_at() {
        let a = Vec3::new(1.0, 2.5, 3.8);
        let b = Vec3::new(2.5, 2.0, 3.0);
        let ray = Ray::new(a, b);
        assert_eq!(format!("{}", ray.at(2 as f64)), "6 6.5 9.8");
    }

    #[test]
    fn hit_sphere_test() {
        let a = Vec3::new(0.0, 0.0, 0.0);
        let b = Vec3::new(-0.075744917850181004, 0.5714285714285714, -1.0);

        let ray = Ray::new(a, b);
        let center = Point3::new(0.0, 0.0, 0.0);
        let low_root = hit_sphere(&center, 0.5, &ray);
        assert_approx_eq!(-0.433185,low_root, 1e-4);
    }
}