mod tuple;

pub use crate::tuple::Tuple;

// TODO: Write equality checker for f64
pub fn float_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.00001
}
