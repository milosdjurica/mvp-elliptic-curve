use crate::point::Point;
use num_bigint::BigUint;

pub struct EllipticCurve {
    a: BigUint,
    #[allow(dead_code)]
    b: BigUint,
    p: BigUint,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CurveError {
    InvalidPoint,
    ScalarIsZero,
}

impl EllipticCurve {
    pub fn new(a: BigUint, b: BigUint, p: BigUint) -> Self {
        EllipticCurve { a, b, p }
    }

    pub fn is_valid_point(&self, point: &Point) -> bool {
        if point.is_infinity() {
            return true;
        }

        let x = point.x.as_ref().unwrap();
        let y = point.y.as_ref().unwrap();

        let left_side = (y * y) % &self.p;
        let right_side = (x * x * x + &self.a * x + &self.b) % &self.p;

        return left_side == right_side;
    }

    pub fn negate_point(&self, point: &Point) -> Result<Point, CurveError> {
        if !self.is_valid_point(point) {
            return Err(CurveError::InvalidPoint);
        }

        if point.is_infinity() {
            return Ok(point.clone());
        }

        let x = point.x.as_ref().unwrap();
        let y = point.y.as_ref().unwrap();

        Ok(Point::new(Some(x.clone()), Some((&self.p - y) % &self.p)))
    }

    pub fn subtract_points(&self, point1: &Point, point2: &Point) -> Result<Point, CurveError> {
        self.add_points(point1, &self.negate_point(point2)?)
    }

    pub fn scalar_division(&self, scalar: BigUint, point: &Point) -> Result<Point, CurveError> {
        if scalar == BigUint::ZERO {
            return Err(CurveError::ScalarIsZero);
        }

        let inverse = self.calculate_inverse(&scalar);
        Ok(self.scalar_multiplication(&inverse, point)?)
    }

    pub fn scalar_multiplication(
        &self,
        scalar: &BigUint,
        point: &Point,
    ) -> Result<Point, CurveError> {
        if !self.is_valid_point(point) {
            return Err(CurveError::InvalidPoint);
        }

        let mut result = Point::new(None, None);
        let mut current = point.clone();
        let mut k = scalar.clone();

        while k > BigUint::from(0u32) {
            if &k % 2u32 == BigUint::from(1u32) {
                result = self.add_points(&result, &current)?;
            }
            current = self.add_points(&current, &current)?;
            k >>= 1;
        }

        Ok(result)
    }

    pub fn add_points(&self, point1: &Point, point2: &Point) -> Result<Point, CurveError> {
        if !self.is_valid_point(point1) || !self.is_valid_point(point2) {
            return Err(CurveError::InvalidPoint);
        }

        // Adding points at infinity
        if point1.is_infinity() {
            return Ok(point2.clone());
        }

        if point2.is_infinity() {
            return Ok(point1.clone());
        }

        let (x1, y1) = (point1.x.as_ref().unwrap(), point1.y.as_ref().unwrap());
        let (x2, y2) = (point2.x.as_ref().unwrap(), point2.y.as_ref().unwrap());

        // Adding inverse
        if x1 == x2 && (y1 + y2) % &self.p == BigUint::from(0u32) {
            return Ok(Point::new(None, None));
        }

        let slope = self.calculate_slope(x1, y1, x2, y2);

        let x3 = (slope.clone() * slope.clone() + &self.p - x1 + &self.p - x2) % &self.p;
        let y3 = (slope * (x1 + &self.p - &x3) + &self.p - y1) % &self.p;

        Ok(Point::new(Some(x3), Some(y3)))
    }

    pub fn order_of_point(&self, point: &Point) -> Result<BigUint, CurveError> {
        if !self.is_valid_point(point) {
            return Err(CurveError::InvalidPoint);
        }

        let mut k = BigUint::from(1u32);
        let mut current = point.clone();

        while !current.is_infinity() {
            current = self.add_points(&current, point)?;
            k += 1u32;
        }

        Ok(k)
    }

    fn calculate_slope(&self, x1: &BigUint, y1: &BigUint, x2: &BigUint, y2: &BigUint) -> BigUint {
        let numerator;
        let denominator;
        if x1 == x2 && y1 == y2 {
            // Point doubling
            numerator = (BigUint::from(3u32) * x1 * x1 + &self.a) % &self.p;
            denominator = self.calculate_inverse(&(BigUint::from(2u32) * y1));
        } else {
            // Point addition
            numerator = (y2 + &self.p - y1) % &self.p;
            denominator = self.calculate_inverse(&(x2 + &self.p - x1));
        }
        (numerator * denominator) % &self.p
    }

    pub fn calculate_inverse(&self, number_to_invert: &BigUint) -> BigUint {
        // ! TODO -> MAYBE HERE SHOULD THROW ERROR IF NUMBER IS 0 !!! Check after in with add_points method
        number_to_invert.modpow(&(&self.p - BigUint::from(2u32)), &self.p)
    }
}
