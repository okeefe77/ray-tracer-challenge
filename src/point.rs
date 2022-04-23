use crate::vector::Vector;
use std::ops::Add;

#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    w: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z, w: 1.0 }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON
            && (self.y - other.y).abs() < f64::EPSILON
            && (self.z - other.z).abs() < f64::EPSILON
            && (self.w - other.w).abs() < f64::EPSILON
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_creates_point() {
        assert_eq!(
            Point {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 1.0
            },
            Point::new(4.0, -4.0, 3.0)
        )
    }

    #[test]
    fn add_vector_to_point() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        let result = Point::new(1.0, 1.0, 6.0);

        assert_eq!(result, p + v)
    }
}
