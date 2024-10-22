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
    fn test_point_not_valid() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(Some(to_biguint(0)), Some(to_biguint(0)));

        assert!(!curve.is_valid_point(&point))
    }

    #[test]
    fn test_negate_point_not_valid() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(Some(to_biguint(0)), Some(to_biguint(0)));

        let negated = curve.negate_point(&point);

        assert!(negated.is_err());
        assert_eq!(negated.unwrap_err(), CurveError::InvalidPoint);
    }

    #[test]
    fn test_negate_point_at_infinity() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(None, None);

        let negated = curve.negate_point(&point).unwrap();

        assert!(negated.is_infinity());
    }

    #[test]
    fn test_negate_negates_correctly_1() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));

        let expected_negated = Point::new(Some(to_biguint(0)), Some(to_biguint(87)));
        let negated = curve.negate_point(&point).unwrap();
        assert_eq!(expected_negated, negated);
    }

    #[test]
    fn test_negate_negates_correctly_2() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(Some(to_biguint(0)), Some(to_biguint(87)));

        let expected_negated = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));
        let negated = curve.negate_point(&point).unwrap();
        assert_eq!(expected_negated, negated);
    }

    #[test]
    fn test_negate_negates_correctly_3() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(Some(to_biguint(3)), Some(to_biguint(91)));

        let expected_negated = Point::new(Some(to_biguint(3)), Some(to_biguint(6)));
        let negated = curve.negate_point(&point).unwrap();
        assert_eq!(expected_negated, negated);
    }

    #[test]
    fn test_subtract_points_invalid_1() {
        let curve = create_curve(2, 3, 97);
        let point1 = Point::new(Some(to_biguint(0)), Some(to_biguint(0)));
        let point2 = Point::new(Some(to_biguint(3)), Some(to_biguint(6)));

        let result = curve.subtract_points(&point1, &point2);

        assert!(result.clone().is_err());
        assert_eq!(result.unwrap_err(), CurveError::InvalidPoint);
    }

    #[test]
    fn test_subtract_points_invalid_2() {
        let curve = create_curve(2, 3, 97);
        let point1 = Point::new(Some(to_biguint(3)), Some(to_biguint(6)));
        let point2 = Point::new(Some(to_biguint(0)), Some(to_biguint(0)));

        let result = curve.subtract_points(&point1, &point2);

        assert!(result.clone().is_err());
        assert_eq!(result.unwrap_err(), CurveError::InvalidPoint);
    }

    #[test]
    fn test_subtract_points_success() {
        let curve = create_curve(2, 3, 97);
        let point1 = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));
        let point2 = Point::new(Some(to_biguint(3)), Some(to_biguint(6)));

        let expected_result = Point::new(Some(to_biguint(47)), Some(to_biguint(79)));
        let result = curve.subtract_points(&point1, &point2).unwrap();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_scalar_division_scalar_is_zero() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));
        let scalar = to_biguint(0);

        let result = curve.scalar_division(scalar, &point);

        assert!(result.clone().is_err());
        assert_eq!(result.unwrap_err(), CurveError::ScalarIsZero);
    }

    #[test]
    fn test_scalar_division_invalid_point() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(Some(to_biguint(0)), Some(to_biguint(0)));
        let scalar = to_biguint(10);

        let result = curve.scalar_division(scalar, &point);

        assert!(result.clone().is_err());
        assert_eq!(result.unwrap_err(), CurveError::InvalidPoint);
    }

    #[test]
    fn test_scalar_division_success() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(Some(to_biguint(0)), Some(to_biguint(0)));
        let scalar = to_biguint(10);

        let inverse = curve.calculate_inverse(&scalar);
        let expected_result = curve.scalar_multiplication(&inverse, &point);
        let result = curve.scalar_division(scalar, &point);

        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_scalar_multiplication_invalid_point() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(Some(to_biguint(0)), Some(to_biguint(0)));
        let scalar = to_biguint(10);

        let result = curve.scalar_multiplication(&scalar, &point);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CurveError::InvalidPoint);
    }

    #[test]
    fn test_scalar_multiplication_scalar_is_zero() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));
        let scalar = to_biguint(0);

        let result = curve.scalar_multiplication(&scalar, &point).unwrap();

        assert_eq!(result, Point::new(None, None));
    }

    #[test]
    fn test_scalar_multiplication_success() {
        let curve = create_curve(2, 3, 97);
        let point = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));
        let scalar = to_biguint(11);

        let result = curve.scalar_multiplication(&scalar, &point).unwrap();

        assert_eq!(
            result,
            Point::new(Some(to_biguint(17)), Some(to_biguint(87)))
        );
    }

    #[test]
    fn test_add_points_invalid_point1() {
        let curve = create_curve(2, 3, 97);
        let point1 = Point::new(Some(to_biguint(0)), Some(to_biguint(0)));
        let point2 = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));

        let result = curve.add_points(&point1, &point2);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CurveError::InvalidPoint);
    }

    #[test]
    fn test_add_points_invalid_point2() {
        let curve = create_curve(2, 3, 97);
        let point1 = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));
        let point2 = Point::new(Some(to_biguint(0)), Some(to_biguint(0)));

        let result = curve.add_points(&point1, &point2);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CurveError::InvalidPoint);
    }

    #[test]
    fn test_add_points_point1_is_infinity() {
        let curve = create_curve(2, 3, 97);
        let point1 = Point::new(None, None);
        let point2 = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));

        let result = curve.add_points(&point1, &point2);

        assert_eq!(result.unwrap(), point2);
    }

    #[test]
    fn test_add_points_point2_is_infinity() {
        let curve = create_curve(2, 3, 97);
        let point1 = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));
        let point2 = Point::new(None, None);

        let result = curve.add_points(&point1, &point2);

        assert_eq!(result.unwrap(), point1);
    }

    #[test]
    fn test_add_points_adds_1() {
        let curve = create_curve(2, 3, 97);
        let point1 = Point::new(Some(to_biguint(3)), Some(to_biguint(6)));
        let point2 = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));

        let expected_result = Point::new(Some(to_biguint(85)), Some(to_biguint(71)));
        let result = curve.add_points(&point1, &point2);

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_add_points_adds_2() {
        let curve = create_curve(2, 3, 97);
        let point1 = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));
        let point2 = Point::new(Some(to_biguint(3)), Some(to_biguint(6)));

        let expected_result = Point::new(Some(to_biguint(85)), Some(to_biguint(71)));
        let result = curve.add_points(&point1, &point2);

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_add_points_adds_3() {
        let curve = create_curve(2, 3, 97);
        let point1 = Point::new(Some(to_biguint(0)), Some(to_biguint(87)));
        let point2 = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));

        let expected_result = Point::new(None, None);
        let result = curve.add_points(&point1, &point2);

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_add_points_adds_4() {
        let curve = create_curve(2, 3, 97);
        let point1 = Point::new(Some(to_biguint(23)), Some(to_biguint(73)));
        let point2 = Point::new(Some(to_biguint(23)), Some(to_biguint(24)));

        let expected_result = Point::new(None, None);
        let result = curve.add_points(&point1, &point2);

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_add_points_adds_5() {
        let curve = create_curve(2, 3, 97);
        let point1 = Point::new(Some(to_biguint(23)), Some(to_biguint(73)));
        let point2 = Point::new(Some(to_biguint(23)), Some(to_biguint(73)));

        let expected_result = Point::new(Some(to_biguint(95)), Some(to_biguint(31)));
        let result = curve.add_points(&point1, &point2);

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_add_points_adds_6() {
        let curve = create_curve(2, 3, 97);
        let point1 = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));
        let point2 = Point::new(Some(to_biguint(0)), Some(to_biguint(10)));

        let expected_result = Point::new(Some(to_biguint(65)), Some(to_biguint(32)));
        let result = curve.add_points(&point1, &point2);

        assert_eq!(result.unwrap(), expected_result);
    }
}
