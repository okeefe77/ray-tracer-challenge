use crate::util::ApproxEq;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r.approx_eq(&other.r) && self.g.approx_eq(&other.g) && self.b.approx_eq(&other.b)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_creates_a_color() {
        let c = Color {
            r: 0.1,
            g: 0.4,
            b: 0.75,
        };

        assert_eq!(c, Color::new(0.1, 0.4, 0.75))
    }

    #[test]
    fn add_two_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let result = Color::new(1.6, 0.7, 1.0);

        assert_eq!(result, c1 + c2)
    }

    #[test]
    fn subtract_two_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let result = Color::new(0.2, 0.5, 0.5);

        assert_eq!(result, c1 - c2)
    }

    #[test]
    fn multiply_color_by_a_scalar() {
        let c = Color::new(0.9, 0.6, 0.75);
        let result = Color::new(1.8, 1.2, 1.5);

        assert_eq!(result, c * 2.0)
    }

    #[test]
    fn multiply_two_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 1.0);
        let result = Color::new(0.9, 0.2, 0.4);

        assert_eq!(result, c1 * c2)
    }
}
