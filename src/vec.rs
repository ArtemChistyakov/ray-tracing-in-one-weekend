use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::raytracing::{random_double, random_double_range};

pub type Point3 = Vec3;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }
    pub fn random_in_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(random_double_range(min, max), random_double_range(min, max), random_double_range(min, max))
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let ran = Vec3::random_in_range(-1.0, 1.0);
            if ran.length_squared() < 1.0 {
                return ran;
            }
        }
    }
    pub fn random_unit_vector() -> Vec3 {
        unit_vector(Vec3::random_in_unit_sphere())
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(&v, &n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(&-uv, &n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
    r_out_perp + r_out_parallel
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(v1.y * v2.z - v1.z * v2.y,
              v1.z * v2.x - v1.x * v2.z,
              v1.x * v2.y - v1.y * v2.x,
    )
}

pub fn unit_vector(vec: Vec3) -> Vec3 {
    vec / vec.length()
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_double_range(-1.0, 1.0),
                          random_double_range(-1.0, 1.0), 0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}


impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec3::new(self.x * rhs as f64, self.y * rhs as f64, self.z * rhs as f64)
    }
}

impl Mul<Vec3> for i32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.x * self as f64,
                  rhs.y * self as f64,
                  rhs.z * self as f64)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::{vec, Vec3};

    #[test]
    fn check_display() {
        let vec3: Vec3 = Vec3::new(1.1, 2.1, 3.1);
        assert_eq!(format!("{}", vec3), "1.1 2.1 3.1");
    }

    #[test]
    fn check_mul_vec3_on_f64() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);
        let mul_vec = vec3 * 1.5;
        assert_eq!(format!("{}", mul_vec), "1.5 3 4.5");
    }

    #[test]
    fn check_mul_f64_on_vec3() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);
        let mul_vec = 1.5 * vec3;
        assert_eq!(format!("{}", mul_vec), "1.5 3 4.5");
    }

    #[test]
    fn check_division_vec3_on_f64() {
        let vec3 = Vec3::new(1.0, 2.0, -3.0);
        let div_vec = vec3 / 2.0;
        assert_approx_eq!(0.5,div_vec.x, 1e-4);
        assert_approx_eq!(1.0,div_vec.y, 1e-4);
        assert_approx_eq!(-1.5,div_vec.z, 1e-4);
    }


    #[test]
    fn check_add_impl() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);
        let vec3_2 = Vec3::new(1.5, 3.3, 0.1);
        let added_vec = vec3 + vec3_2;
        assert_eq!(format!("{}", added_vec), "2.5 5.3 3.1");
    }

    #[test]
    fn check_subtraction_vec3() {
        let vec0 = Vec3::new(1.0, 1.5, 2.7);
        let vec1 = Vec3::new(0.5, 2.0, 1.0);
        let sub_vec = vec0 - vec1;
        assert_approx_eq!(0.5,sub_vec.x, 1e-4);
        assert_approx_eq!(-0.5,sub_vec.y, 1e-4);
        assert_approx_eq!(1.7,sub_vec.z, 1e-4);
    }

    #[test]
    fn check_dot() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);
        let vec3_2 = Vec3::new(1.5, 3.3, 0.1);
        let f = vec::dot(&vec3, &vec3_2);
        assert_approx_eq!(8.4, f, 1e-4);
    }

    #[test]
    fn check_unit_vector() {
        let vec3 = Vec3::new(1.0, 2.0, 2.0);
        let unit_vec = vec::unit_vector(vec3);
        assert_approx_eq!(vec3.x/3.0, unit_vec.x, 1e-4);
        assert_approx_eq!(vec3.y/3.0, unit_vec.y, 1e-4);
        assert_approx_eq!(vec3.z/3.0, unit_vec.z, 1e-4);
    }
}