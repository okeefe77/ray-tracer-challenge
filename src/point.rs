use crate::util::ApproxEq;
use crate::vector::Vector;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug)]
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
        self.x.approx_eq(&other.x)
            && self.y.approx_eq(&other.y)
            && self.z.approx_eq(&other.z)
            && self.w.approx_eq(&other.w)
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, rhs: Point) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, rhs: Vector) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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

    #[test]
    fn subtract_two_points() {
        let p1 = Point::new(3.0, -2.0, 5.0);
        let p2 = Point::new(-2.0, 3.0, 1.0);
        let result = Vector::new(5.0, -5.0, 4.0);

        assert_eq!(result, p1 - p2)
    }

    #[test]
    fn subtract_vector_from_point() {
        let p = Point::new(3.0, -2.0, 5.0);
        let v = Vector::new(-2.0, 3.0, 1.0);
        let result = Point::new(5.0, -5.0, 4.0);

        assert_eq!(result, p - v)
    }
}
