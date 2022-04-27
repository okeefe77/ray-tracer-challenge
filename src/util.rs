pub trait ApproxEq {
    fn approx_eq(&self, other: &Self) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(&self, other: &Self) -> bool {
        (self - other).abs() < Self::EPSILON
    }
}
