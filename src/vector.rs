use crate::point::Point;
use std::ops::Add;

#[derive(Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    w: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z, w: 0.0 }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < f64::EPSILON
            && (self.y - other.y).abs() < f64::EPSILON
            && (self.z - other.z).abs() < f64::EPSILON
            && (self.w - other.w).abs() < f64::EPSILON
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_creates_a_vector() {
        assert_eq!(
            Vector {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 0.0
            },
            Vector::new(4.0, -4.0, 3.0)
        )
    }

    #[test]
    fn add_two_vectors() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);
        let result = Vector::new(1.0, 1.0, 6.0);

        assert_eq!(result, v1 + v2)
    }

    #[test]
    fn add_point_to_vector() {
        let v = Vector::new(3.0, -2.0, 5.0);
        let p = Point::new(-2.0, 3.0, 1.0);
        let result = Point::new(1.0, 1.0, 6.0);

        assert_eq!(result, v + p)
    }
}
