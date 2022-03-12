use std::f64::consts::PI;
use std::rc::Rc;

use rand::Rng;

use crate::{vec, Vec3};
use crate::color::Color;
use crate::vec::Point3;

#[derive(Default)]
pub struct HitRecord {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}


impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = vec::dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
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
            if let Some(hit_rec) = object.hit(r, t_min, t_max) {
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
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
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
        let root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }
        let mut rec = HitRecord::default();
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

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


pub fn ray_color(ray: &Ray, world: &dyn Hittable,depth: usize) -> Color {
    if depth == 0 {
        return Color::new(0.0,0.0,0.0);
    }
    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        let target = hit_record.p + hit_record.normal + Vec3::random_unit_vector();
        let new_ray = Ray::new(hit_record.p,target-hit_record.p);
        return 0.5 * ray_color(&new_ray,world,depth-1);
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

    use crate::raytracing::{ Ray};
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