use crate::point::Point;
use num_bigint::{BigUint, ToBigUint};

pub struct EllipticCurve {
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

impl EllipticCurve {
    fn new(a: BigUint, b: BigUint, p: BigUint) -> Self {
        EllipticCurve { a, b, p }
    }

    fn point_add(&self, p1: &Point, p2: &Point) -> Point {
        if p1.is_infinity() {
            return p2.clone();
        }

        if p2.is_infinity() {
            return p1.clone();
        }

        // ! TODO -> change
        Point::new(0.to_biguint(), 0.to_biguint())
    }
}
