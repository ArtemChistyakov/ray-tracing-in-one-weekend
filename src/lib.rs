use std::fmt::{Display, Formatter, write};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3<T: Copy + Display> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T>
    where T: Copy + Display {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

impl<T> Display for Vec3<T>
    where
        T: Display + Copy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use crate::Vec3;

    #[test]
    fn check_display() {
        let vec3: Vec3<f64> = Vec3::new(1.1, 2.1, 3.1);
        assert_eq!(format!("{}", vec3), "1.1 2.1 3.1");
    }
}