use crate::point::Point;
use crate::util::ApproxEq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
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

    pub fn magnitude(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }

    pub fn normalize(&self) -> Vector {
        self.clone() / self.magnitude()
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x.approx_eq(&other.x)
            && self.y.approx_eq(&other.y)
            && self.z.approx_eq(&other.z)
            && self.w.approx_eq(&other.w)
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

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector::new(self.x / rhs, self.y / rhs, self.z / rhs)
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

    #[test]
    fn subtract_two_vectors() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);
        let result = Vector::new(5.0, -5.0, 4.0);

        assert_eq!(result, v1 - v2)
    }

    #[test]
    fn negate_vector() {
        let v = Vector::new(3.0, -2.0, 5.0);
        let result = Vector::new(-3.0, 2.0, -5.0);

        assert_eq!(result, -v)
    }

    #[test]
    fn multiply_vector_by_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);
        let t = 3.5;
        let result = Vector::new(3.5, -7.0, 10.5);

        assert_eq!(result, v * t)
    }

    #[test]
    fn multiply_vector_by_fractional_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);
        let t = 0.5;
        let result = Vector::new(0.5, -1.0, 1.5);

        assert_eq!(result, v * t)
    }

    #[test]
    fn divide_vector_by_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);
        let t = 2.0;
        let result = Vector::new(0.5, -1.0, 1.5);

        assert_eq!(result, v / t)
    }

    #[test]
    fn magnitude_of_x_unit_vector_is_one() {
        let x_unit = Vector::new(1.0, 0.0, 0.0);
        assert!(x_unit.magnitude().approx_eq(&1.0))
    }

    #[test]
    fn magnitude_of_y_unit_vector_is_one() {
        let y_unit = Vector::new(0.0, 1.0, 0.0);
        assert!(y_unit.magnitude().approx_eq(&1.0))
    }

    #[test]
    fn magnitude_of_z_unit_vector_is_one() {
        let z_unit = Vector::new(0.0, 0.0, 1.0);
        assert!(z_unit.magnitude().approx_eq(&1.0))
    }

    #[test]
    fn magnitude_of_positive_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let result = f64::sqrt(14.0);

        assert!(v.magnitude().approx_eq(&result))
    }

    #[test]
    fn magnitude_of_negative_vector() {
        let v = Vector::new(-1.0, -2.0, -3.0);
        let result = f64::sqrt(14.0);

        assert!(v.magnitude().approx_eq(&result))
    }

    #[test]
    fn normalize_horizontal_vector() {
        let v = Vector::new(4.0, 0.0, 0.0);
        let result = Vector::new(1.0, 0.0, 0.0);

        assert_eq!(result, v.normalize())
    }

    #[test]
    fn normalize_vector() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let t = f64::sqrt(14.0);
        let result = Vector::new(1.0 / t, 2.0 / t, 3.0 / t);

        assert_eq!(result, v.normalize())
    }

    #[test]
    fn normalized_vector_has_magnitude_of_one() {
        let v = Vector::new(1.0, 2.0, 3.0);
        assert!(v.normalize().magnitude().approx_eq(&1.0))
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        assert!(v1.dot(&v2).approx_eq(&20.0))
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let result_a = Vector::new(-1.0, 2.0, -1.0);
        let result_b = Vector::new(1.0, -2.0, 1.0);

        assert_eq!(result_a, v1.cross(&v2));
        assert_eq!(result_b, v2.cross(&v1));
    }
}
