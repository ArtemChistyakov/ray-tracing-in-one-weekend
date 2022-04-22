use std::f64::consts::PI;
use std::rc::Rc;

use rand::Rng;

use crate::{vec, Vec3};
use crate::color::Color;
use crate::material::Scatter;
use crate::vec::Point3;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn Scatter>,
    pub t: f64,
    pub front_face: bool,
}


impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray) {
        self.front_face = vec::dot(&r.direction(), &self.normal) < 0.0;
        self.normal = if self.front_face {
            self.normal
        } else {
            -self.normal
        };
    }
    pub fn new(p: Point3, normal: Vec3, mat_ptr: Rc<dyn Scatter>, t: f64) -> HitRecord {
        HitRecord { p, normal, mat_ptr, t, front_face: false }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new()
        }
    }
    pub fn with_capacity(initial_capacity: usize) -> HittableList {
        HittableList {
            objects: Vec::with_capacity(initial_capacity)
        }
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(hit_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_rec.t;
                temp_rec = Some(hit_rec);
            }
        }
        temp_rec
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Rc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat_ptr: Rc<dyn Scatter>) -> Sphere {
        Sphere { center, radius, mat_ptr }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = vec::dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;
        let mat_ptr = Rc::clone(&self.mat_ptr);
        let mut rec = HitRecord::new(p, outward_normal, mat_ptr, t);
        rec.set_face_normal(r);

        Some(rec)
    }
}


pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { orig: a, dir: b }
    }

    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + (self.dir * t)
    }
}


pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((scattered, attenuation)) = hit_record.mat_ptr.scatter(ray, &hit_record) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = vec::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}


#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::raytracing::Ray;
    use crate::Vec3;
    use crate::vec::Point3;

    #[test]
    fn check_at() {
        let a = Vec3::new(1.0, 2.5, 3.8);
        let b = Vec3::new(2.5, 2.0, 3.0);
        let ray = Ray::new(a, b);
        assert_eq!(format!("{}", ray.at(2 as f64)), "6 6.5 9.8");
    }
}