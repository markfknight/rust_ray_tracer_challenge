use crate::float_eq;

use std::ops::Add;

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

    // Should this be moved to the outer scope?
    fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: 1.0,
        }
    }

    // Should this be moved to the outer scope?
    fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
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

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_hold_correct_x_value_in_point_tuple() {
        let point = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(point.x, 4.3);
    }

    #[test]
    fn should_hold_correct_y_value_in_point_tuple() {
        let point = Tuple {
            x: 4.3,
            y: -4.3,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(point.y, -4.3)
    }

    #[test]
    fn should_hold_correct_z_value_in_point_tuple() {
        let point = Tuple {
            x: 4.3,
            y: -4.3,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(point.z, 3.1);
    }

    #[test]
    fn should_hold_correct_w_value_in_point_tuple() {
        let point = Tuple {
            x: 4.3,
            y: -4.3,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(point.w, 1.0);
    }

    #[test]
    fn should_be_a_point_when_w_equal_to_1() {
        let point = Tuple {
            x: 4.3,
            y: -4.3,
            z: 3.2,
            w: 1.0,
        };

        assert!(point.is_point())
    }

    #[test]
    fn should_not_be_a_vector_when_w_equal_to_1() {
        let point = Tuple {
            x: 4.3,
            y: -4.3,
            z: 3.2,
            w: 1.0,
        };

        assert!(!point.is_vector())
    }

    #[test]
    fn should_hold_correct_x_value_in_vector_tuple() {
        let vector = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(vector.x, 4.3);
    }

    #[test]
    fn should_hold_correct_y_value_in_vector_tuple() {
        let vector = Tuple {
            x: 4.3,
            y: -4.3,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(vector.y, -4.3)
    }

    #[test]
    fn should_hold_correct_z_value_in_vector_tuple() {
        let vector = Tuple {
            x: 4.3,
            y: -4.3,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(vector.z, 3.1);
    }

    #[test]
    fn should_hold_correct_w_value_in_vector_tuple() {
        let vector = Tuple {
            x: 4.3,
            y: -4.3,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(vector.w, 1.0);
    }

    #[test]
    fn should_not_be_a_point_when_w_equal_to_0() {
        let vector = Tuple {
            x: 4.3,
            y: -4.3,
            z: 3.2,
            w: 0.0,
        };

        assert!(!vector.is_point())
    }

    #[test]
    fn should_be_a_vector_when_w_equal_to_0() {
        let vector = Tuple {
            x: 4.3,
            y: -4.3,
            z: 3.2,
            w: 0.0,
        };

        assert!(vector.is_vector())
    }

    #[test]
    fn should_return_a_tuple_with_w_equal_to_1() {
        let point = Tuple::point(4.0, -4.0, 3.0);

        assert_eq!(
            point,
            Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 1.0
            }
        )
    }

    #[test]
    fn should_return_a_tuple_with_w_equal_to_0() {
        let point = Tuple::vector(4.0, -4.0, 3.0);

        assert_eq!(
            point,
            Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 0.0
            }
        )
    }

    #[test]
    fn should_return_the_correct_value_when_two_tuples_are_added_together() {
        let a1 = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0
        };

        let a2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0
        };

        assert_eq!(a1 + a2, Tuple { x: 1.0, y: 1.0, z: 6.0, w: 1.0 })
    }
}
