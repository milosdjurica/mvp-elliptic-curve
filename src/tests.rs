#[cfg(test)]
mod tests {
    use num_bigint::{BigUint, ToBigUint};

    use crate::{
        elliptic_curve::EllipticCurve,
        point::{self, Point},
    };

    fn create_curve(a: u32, b: u32, p: u32) -> EllipticCurve {
        EllipticCurve::new(to_biguint(a), to_biguint(b), to_biguint(p))
    }

    fn to_biguint(val: u32) -> BigUint {
        BigUint::try_from(val).unwrap()
    }

    #[test]
    fn test_negate_point_at_infinity() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(None, None);

        let negated = curve.negate_point(&point);

        assert!(negated.is_infinity());
    }
}
