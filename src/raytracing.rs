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
    let oc: Vec3 = calc_oc(r.origin().clone(), center.clone());
    let a = calc_a(r.direction());
    let b = calc_b(&oc, r.direction());
    let c = calc_c(&oc, radius);
    let discriminant = calc_discriminant(a, b, c);
    return if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    };
}

pub fn hit_sphere_old(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc: Vec3 = calc_oc(r.origin().clone(), center.clone());
    let a = calc_a(r.direction());
    let b = calc_b(&oc, r.direction());
    let c = calc_c(&oc, radius);
    let discriminant = calc_discriminant(a, b, c);
    return discriminant > 0.0;
}

pub fn calc_discriminant(a: f64, b: f64, c: f64) -> f64 {
    b * b - 4.0 * a * c
}


fn calc_oc(origin: Vec3, center: Vec3) -> Vec3 {
    origin - center
}

fn calc_a(direction: &Vec3) -> f64 {
    vec::dot(direction, direction)
}

fn calc_b(oc: &Vec3, direction: &Vec3) -> f64 {
    2.0 * vec::dot(oc, direction)
}

fn calc_c(oc: &Vec3, radius: f64) -> f64 {
    vec::dot(oc, oc) - radius * radius
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

pub fn ray_color_old(ray: &Ray) -> Color {
    let cen = Point3::new(0.0, 0.0, -1.0);
    if hit_sphere_old(&cen, 0.5, &ray) {
        return Color::new(1.0, 0.0, 0.0);
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
    fn check_calc_discriminant() {
        let a = 10.0;
        let b = 5.0;
        let c = 1.0;
        let res = calc_discriminant(a, b, c);
        assert_approx_eq!(-15.0,res, 1e-4);
    }

    #[test]
    fn check_calc_res() {
        let a = 10.0;
        let b = 5.0;
        let disc = 4.0f64;
        let res = (-b - disc.sqrt()) / (2.0 * a);
        assert_approx_eq!(-0.35,res, 1e-4);
    }

    #[test]
    fn check_calc_oc() {
        let origin = Point3::new(0.0, 0.0, 0.0);
        let cen = Point3::new(0.0, 0.0, -1.0);
        let oc = calc_oc(origin, cen);
        assert_approx_eq!(0.0,oc.x, 1e-4);
        assert_approx_eq!(0.0,oc.y, 1e-4);
        assert_approx_eq!(1.0,oc.z, 1e-4);
    }

    #[test]
    fn check_calc_a() {
        let direction = Vec3::new(1.0, 2.0, 3.0);
        let f = calc_a(&direction);
        assert_approx_eq!(14.0, f, 1e-4);
    }

    #[test]
    fn check_calc_b() {
        let oc = Point3::new(0.0, 0.0, 1.0);
        let direction = Point3::new(2.0, 3.0, 4.0);
        let res = calc_b(&oc, &direction);
        assert_approx_eq!(8.0,res, 1e-4);
    }

    #[test]
    fn check_calc_c() {
        let oc = Point3::new(0.0, 0.0, 1.0);
        let radius = 0.5;
        let res = calc_c(&oc, radius);
        assert_approx_eq!(0.75,res, 1e-4);
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