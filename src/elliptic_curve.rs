use crate::point::Point;
use num_bigint::BigUint;

pub struct EllipticCurve {
    a: BigUint,
    #[allow(dead_code)]
    b: BigUint,
    p: BigUint,
}

#[derive(Debug)]
pub enum CurveError {
    InvalidPoint,
}

impl EllipticCurve {
    pub fn new(a: BigUint, b: BigUint, p: BigUint) -> Self {
        EllipticCurve { a, b, p }
    }

    pub fn negate_point(&self, point: &Point) -> Result<Point, CurveError> {
        if !self.ensure_point_is_valid(point) {
            return Err(CurveError::InvalidPoint);
        }

        if point.is_infinity() {
            return Ok(point.clone());
        }

        let x = point.x.as_ref().unwrap();
        let y = point.y.as_ref().unwrap();

        Ok(Point::new(Some(x.clone()), Some((&self.p - y) % &self.p)))
    }

    pub fn ensure_point_is_valid(&self, point: &Point) -> bool {
        if point.is_infinity() {
            return true;
        }

        let x = point.x.as_ref().unwrap();
        let y = point.y.as_ref().unwrap();

        let left_side = (y * y) % &self.p;
        let right_side = (x * x * x + &self.a * x + &self.b) % &self.p;

        return left_side == right_side;
    }

    pub fn subtract_points(&self, point1: &Point, point2: &Point) -> Result<Point, CurveError> {
        Ok(self.add_points(point1, &self.negate_point(point2)?))
    }

    pub fn scalar_division(&self, scalar: BigUint, point: &Point) -> Option<Point> {
        if scalar == BigUint::ZERO {
            return None;
        }

        let inverse = self.calculate_inverse(&scalar);
        Some(self.scalar_multiplication(&inverse, point))
    }

    pub fn scalar_multiplication(&self, scalar: &BigUint, point: &Point) -> Point {
        let mut result = Point::new(None, None);
        let mut current = point.clone();
        let mut k = scalar.clone();

        while k > BigUint::from(0u32) {
            if &k % 2u32 == BigUint::from(1u32) {
                result = self.add_points(&result, &current);
            }
            current = self.add_points(&current, &current);
            k >>= 1;
        }

        result
    }

    pub fn add_points(&self, point1: &Point, point2: &Point) -> Point {
        // Adding points at infinity
        if point1.is_infinity() {
            return point2.clone();
        }

        if point2.is_infinity() {
            return point1.clone();
        }

        let (x1, y1) = (point1.x.as_ref().unwrap(), point1.y.as_ref().unwrap());
        let (x2, y2) = (point2.x.as_ref().unwrap(), point2.y.as_ref().unwrap());

        // Adding inverse
        if x1 == x2 && (y1 + y2) % &self.p == BigUint::from(0u32) {
            return Point::new(None, None);
        }

        let slope = self.calculate_slope(x1, y1, x2, y2);

        let x3 = (slope.clone() * slope.clone() + &self.p - x1 + &self.p - x2) % &self.p;
        let y3 = (slope * (x1 + &self.p - &x3) + &self.p - y1) % &self.p;

        Point::new(Some(x3), Some(y3))
    }

    pub fn order_of_point(&self, point: &Point) -> BigUint {
        let mut k = BigUint::from(1u32);
        let mut current = point.clone();

        while !current.is_infinity() {
            current = self.add_points(&current, point);
            k += 1u32;
        }

        k
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

    fn calculate_inverse(&self, number_to_invert: &BigUint) -> BigUint {
        number_to_invert.modpow(&(&self.p - BigUint::from(2u32)), &self.p)
    }
}
