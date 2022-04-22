use core::f64;

use crate::{raytracing, vec, Vec3};
use crate::color::Color;
use crate::raytracing::{HitRecord, Ray};

pub trait Scatter {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric {
            ir
        }
    }
    fn reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        let r0 = r0 * r0;
        return r0 + (1.0 - r0) * ((1.0 - cos_theta).powi(5));
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = vec::unit_vector(r_in.direction());
        let cos_theta = f64::min(vec::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = (refraction_ratio * sin_theta) > 1.0;
        let direction;
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > raytracing::random_double() {
            direction = vec::reflect(unit_direction, rec.normal);
        } else {
            direction = vec::refract(unit_direction, rec.normal, refraction_ratio);
        }

        let scattered = Ray::new(rec.p, direction);
        Some((scattered, attenuation))
    }
}


impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        if fuzz < 1.0 {
            Metal { albedo, fuzz }
        } else {
            Metal { albedo, fuzz: 1.0 }
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = vec::reflect(vec::unit_vector(r_in.direction()), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        let attenuation = self.albedo;
        if vec::dot(&scattered.direction(), &rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else { None }
    }
}