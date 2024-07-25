use crate::point::{self, Point};
use num_bigint::BigUint;

pub struct EllipticCurve {
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl EllipticCurve {
    fn new(a: BigUint, b: BigUint, p: BigUint) -> Self {
        EllipticCurve { a, b, p }
    }

    fn add_points(&self, p1: &Point, p2: &Point) -> Point {
        if p1.is_infinity() {
            return p2.clone();
        }

        if p2.is_infinity() {
            return p1.clone();
        }

        let x1 = p1.x.as_ref().unwrap();
        let y1 = p1.y.as_ref().unwrap();
        let x2 = p2.x.as_ref().unwrap();
        let y2 = p2.y.as_ref().unwrap();

        let m;

        if x1 == x2 && y1 == y2 {
            // Point doubling
            let numerator = (BigUint::from(3u32) * x1 * x1 + &self.a) % &self.p;
            let denominator =
                (BigUint::from(2u32) * y1).modpow(&(&self.p - BigUint::from(2u32)), &self.p);

            m = (numerator * denominator) % &self.p;
        } else {
            // Point addition
            let numerator = (y2 - y1 + &self.p) % &self.p;
            let denominator = (x2 - x1 + &self.p).modpow(&(&self.p - BigUint::from(2u32)), &self.p);

            m = (numerator * denominator) % &self.p;
        }

        let x3 = (m.clone() * m.clone() - x1 - x2 + &self.p) % &self.p;
        let y3 = (m * (x1 - &x3) - y1 + &self.p) % &self.p;

        Point::new(Some(x3), Some(y3))
    }

    fn scalar_multiplication(&self, scalar: &BigUint, point: &Point) -> Point {
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
}
