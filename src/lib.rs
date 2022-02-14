use crate::vec::Vec3;

pub mod vec;


pub struct Ray {
    a: Vec3,
    b: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a, b }
    }
    pub fn origin(&self) -> &Vec3 {
        &self.a
    }
    pub fn direction(&self) -> &Vec3 {
        &self.b
    }
    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a.clone() + (self.b.clone() * t)
    }
    pub fn color(&self) -> Vec3 {
        let unit_direction = Vec3::unit_vector(self.direction().clone());
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3 { x: 1.0, y: 1.0, z: 1.0 } * (1.0 - t) + Vec3 { x: 0.5, y: 0.7, z: 1.0 } * t
    }
}

#[cfg(test)]
mod tests {
    use crate::{Ray, Vec3};

    #[test]
    fn check_point_at_param() {
        let a: Vec3 = Vec3::new(1.0, 2.5, 3.8);
        let b: Vec3 = Vec3::new(2.5, 2.0, 3.0);
        let ray = Ray::new(a, b);
        assert_eq!(format!("{}", ray.point_at_parameter(2 as f32)), "6 6.5 9.8");
    }
}
