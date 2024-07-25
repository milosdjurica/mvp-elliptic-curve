use crate::point::Point;
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
}
