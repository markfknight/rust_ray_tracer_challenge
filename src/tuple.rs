use crate::float_eq;

use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn tuple<X: Into<f64>, Y: Into<f64>, Z: Into<f64>, W: Into<f64>>(
        x: X,
        y: Y,
        z: Z,
        w: W,
    ) -> Tuple {
        Tuple {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }

    // Should this be moved to the outer scope?
    pub fn point<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Tuple {
        Tuple::tuple(x, y, z, 1.0)
    }

    // Should this be moved to the outer scope?
    pub fn vector<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Tuple {
        Tuple::tuple(x, y, z, 0.0)
    }
}

// Implemented as we are comparing floats, other wise we could have derived!
impl PartialEq<Tuple> for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        float_eq(self.x, other.x)
            && float_eq(self.y, other.y)
            && float_eq(self.z, other.z)
            && float_eq(self.w, other.w)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_hold_correct_value_in_point_tuple() {
        let point = Tuple::tuple(4.3, -4.2, 3.1, 1.0);

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);
        assert!(point.is_point());
        assert!(!point.is_vector());
    }

    #[test]
    fn should_hold_correct_value_in_vector_tuple() {
        let vector = Tuple::tuple(4.3, -4.2, 3.1, 0.0);

        assert_eq!(vector.x, 4.3);
        assert_eq!(vector.y, -4.2);
        assert_eq!(vector.z, 3.1);
        assert_eq!(vector.w, 0.0);
        assert!(!vector.is_point());
        assert!(vector.is_vector());
    }

    #[test]
    fn should_return_a_tuple_with_w_equal_to_1() {
        let point = Tuple::point(4.0, -4.0, 3.0);

        assert_eq!(point, Tuple::tuple(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn should_return_a_tuple_with_w_equal_to_0() {
        let vector = Tuple::vector(4.0, -4.0, 3.0);

        assert_eq!(vector, Tuple::tuple(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn should_return_the_correct_value_when_two_tuples_are_added() {
        let a1 = Tuple::tuple(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::tuple(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(a1 + a2, Tuple::tuple(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn should_return_the_correct_value_when_two_points_are_subtracted() {
        let a1 = Tuple::point(3.0, 2.0, 1.0);
        let a2 = Tuple::point(5.0, 6.0, 7.0);

        assert_eq!(a1 - a2, Tuple::vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn should_return_the_correct_value_when_a_point_is_subtracted_from_a_vector() {
        let a1 = Tuple::point(3.0, 2.0, 1.0);
        let a2 = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(a1 - a2, Tuple::point(-2.0, -4.0, -6.0))
    }

    #[test]
    fn should_return_the_correct_value_when_two_vectors_are_subtracted() {
        let a1 = Tuple::vector(3.0, 2.0, 1.0);
        let a2 = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(a1 - a2, Tuple::vector(-2.0, -4.0, -6.0))
    }

    #[test]
    fn should_return_correct_vector_when_a_vector_is_subtracted_from_a_zero_tuple() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);

        assert_eq!(zero - v, Tuple::vector(-1.00, 2.0, -3.00));
    }

    #[test]
    fn should_negate_a_tuple_correctly() {
        let a = Tuple::tuple(1.0, -2.0, 3.0, -4.0);

        assert_eq!(-a, Tuple::tuple(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn should_return_correct_tuple_when_multiplied_by_a_scalar() {
        let a = Tuple::tuple(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a * 3.5, Tuple::tuple(3.5, -7.0, 10.5, -14.0))
    }
}
