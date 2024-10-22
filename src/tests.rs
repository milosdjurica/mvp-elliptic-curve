#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use crate::{
        elliptic_curve::{CurveError, EllipticCurve},
        point::Point,
    };

    fn create_curve(a: u32, b: u32, p: u32) -> EllipticCurve {
        EllipticCurve::new(to_biguint(a), to_biguint(b), to_biguint(p))
    }

    fn to_biguint(val: u32) -> BigUint {
        BigUint::try_from(val).unwrap()
    }

    #[test]
    fn test_negate_point_not_valid() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(Some(to_biguint(0)), Some(to_biguint(0)));

        let negated = curve.negate_point(&point);

        assert_eq!(negated.unwrap_err(), CurveError::InvalidPoint);
    }

    #[test]
    fn test_point_not_valid() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(Some(to_biguint(0)), Some(to_biguint(0)));

        assert!(!curve.ensure_point_is_valid(&point))
    }

    #[test]
    fn test_negate_point_at_infinity() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(None, None);

        let negated = curve.negate_point(&point).unwrap();

        assert!(negated.is_infinity());
    }
}
