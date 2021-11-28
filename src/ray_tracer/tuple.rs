use auto_ops::*;
use serde::Deserialize;

fn f64_is_equal(a: f64, b: f64) -> bool {
    let epsilon = 0.00001;
    (a - b).abs() < epsilon
}


#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            w: 1.0,
        }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
            w: 0.0,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Self { 
            x: self.x / magnitude, 
            y: self.y / magnitude, 
            z: self.z / magnitude, 
            w: self.w / magnitude, 
        }
    }

    pub fn dot(&self, rhs: &Tuple) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn cross(&self, rhs: &Tuple) -> Tuple {
        Self { 
            x: self.y * rhs.z - self.z * rhs.y, 
            y: self.z * rhs.x - self.x * rhs.z, 
            z: self.x * rhs.y - self.y * rhs.x, 
            w: 0.0,
        }
    }
}

impl_op_ex!(+ |lhs: &Tuple, rhs: &Tuple| -> Tuple {
        Tuple {
            x: lhs.x + rhs.x,
            y: lhs.y + rhs.y,
            z: lhs.z + rhs.z,
            w: lhs.w + rhs.w,
        }
});
impl_op_ex!(- |lhs: &Tuple, rhs: &Tuple| -> Tuple {
        Tuple {
            x: lhs.x - rhs.x,
            y: lhs.y - rhs.y,
            z: lhs.z - rhs.z,
            w: lhs.w - rhs.w,
        }
});
impl_op_ex!(* |lhs: &Tuple, rhs: &f64| -> Tuple {
        Tuple {
            x: lhs.x * rhs,
            y: lhs.y * rhs,
            z: lhs.z * rhs,
            w: lhs.w * rhs,
        }
});
impl_op_ex!(/ |lhs: &Tuple, rhs: &f64| -> Tuple {
        Tuple {
            x: lhs.x / rhs,
            y: lhs.y / rhs,
            z: lhs.z / rhs,
            w: lhs.w / rhs,
        }
});
impl_op_ex!(- |lhs: &Tuple| -> Tuple {
        Tuple {
            x: -lhs.x,
            y: -lhs.y,
            z: -lhs.z,
            w: -lhs.w,
        }
});

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        f64_is_equal(self.x, other.x) &&
        f64_is_equal(self.y, other.y) &&
        f64_is_equal(self.y, other.y) &&
        f64_is_equal(self.w, other.w)
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;
    use super::*;

    #[test]
    fn test_a_tuple_with_w_1_is_a_point() {
        let a = Tuple::point(4.3, -4.2, 3.1);

        assert_eq!(a.w, 1.0);
    }

    #[test]
    fn test_a_tuple_with_w_0_is_a_vector() {
        let a = Tuple::vector(4.3, -4.2, 3.1);

        assert_eq!(a.w, 0.0);
    }

    #[test]
    fn test_adding_two_tuples() {
        let a1 = Tuple::point(3.0, -2.0, 5.0);
        let a2 = Tuple::vector(-2.0, 3.0, 1.0);

        assert_eq!(a1 + a2, Tuple {x: 1.0, y: 1.0, z: 6.0, w: 1.0});
        assert_eq!(&a1 + a2, Tuple {x: 1.0, y: 1.0, z: 6.0, w: 1.0});
        assert_eq!(&a1 + &a2, Tuple {x: 1.0, y: 1.0, z: 6.0, w: 1.0});
        assert_eq!(a1 + &a2, Tuple {x: 1.0, y: 1.0, z: 6.0, w: 1.0});
    }

    #[test]
    fn test_subtracting_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtracting_a_vector_from_a_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn test_subtracting_a_vector_from_the_zero_vector() {
        let z = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);

        assert_eq!(z - v, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_negating_a_tuple() {
        let a = Tuple {x: 1.0, y: -2.0, z: 3.0, w: -4.0};

        assert_eq!(-a, Tuple {x: -1.0, y: 2.0, z: -3.0, w: 4.0});
    }

    #[test]
    fn test_multiplying_a_tuple_by_a_scalar() {
        let a = Tuple {x: 1.0, y: -2.0, z: 3.0, w: -4.0};

        assert_eq!(a * 3.5, Tuple {x: 3.5, y: -7.0, z: 10.5, w: -14.0});
    }

    #[test]
    fn test_multiplying_a_tuple_by_a_fraction() {
        let a = Tuple {x: 1.0, y: -2.0, z: 3.0, w: -4.0};

        assert_eq!(a * 0.5, Tuple {x: 0.5, y: -1.0, z: 1.5, w: -2.0});
    }

    #[test]
    fn test_dividing_a_tuple_by_a_scalar() {
        let a = Tuple {x: 1.0, y: -2.0, z: 3.0, w: -4.0};

        assert_eq!(a / 2.0, Tuple {x: 0.5, y: -1.0, z: 1.5, w: -2.0});
    }

    #[rstest]
    #[case(1.0, 0.0, 0.0, 1.0)]
    #[case(0.0, 1.0, 0.0, 1.0)]
    #[case(0.0, 0.0, 1.0, 1.0)]
    #[case(1.0, 2.0, 3.0, 14.0_f64.sqrt())]
    #[case(-1.0, -2.0, -3.0, 14.0_f64.sqrt())]
    fn test_computing_the_magnitude_of_vector(#[case] x: f64, #[case] y: f64, #[case] z: f64, #[case]expected: f64) {
        let v = Tuple::vector(x, y, z);

        assert_eq!(v.magnitude(), expected);
    }

    #[rstest]
    #[case(Tuple::vector(4.0, 0.0, 0.0), Tuple::vector(1.0, 0.0, 0.0))]
    #[case(Tuple::vector(1.0, 2.0, 3.0), Tuple::vector(0.26726, 0.53452, 0.80178))]
    fn test_normalizing_vector(#[case] v: Tuple, #[case] expected: Tuple) {
        assert_eq!(v.normalize(), expected);
    }

    #[test]
    fn test_magnitude_of_a_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        assert_eq!(v.normalize().magnitude(), 1.0);
    }

    #[test]
    fn test_dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn test_cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(a.cross(&b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), Tuple::vector(1.0, -2.0, 1.0));
    }
}
