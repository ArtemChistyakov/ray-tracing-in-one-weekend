use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul};

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn make_unit_vector(&self) -> Vec3 {
        let k = 1.0 / f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        Vec3 { x: self.x * k, y: self.y * k, z: self.z * k }
    }
    pub fn unit_vector(vec: Vec3) -> Vec3 {
        vec / vec.length()
    }
    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i32) -> Self::Output {
        Vec3 { x: self.x * rhs as f32, y: self.y * rhs as f32, z: self.z * rhs as f32 }
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

#[cfg(test)]
mod tests {
    use crate::Vec3;

    #[test]
    fn check_display() {
        let vec3: Vec3 = Vec3::new(1.1, 2.1, 3.1);
        assert_eq!(format!("{}", vec3), "1.1 2.1 3.1");
    }

    #[test]
    fn check_mul_impl() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);
        let mul_vec = vec3 * 1.5;
        assert_eq!(format!("{}", mul_vec), "1.5 3 4.5");
    }

    #[test]
    fn check_add_impl() {
        let vec3 = Vec3::new(1.0, 2.0, 3.0);
        let vec3_2 = Vec3::new(1.5, 3.3, 0.1);
        let added_vec = vec3 + vec3_2;
        assert_eq!(format!("{}", added_vec), "2.5 5.3 3.1");
    }
}