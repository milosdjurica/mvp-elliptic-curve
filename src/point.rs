use num_bigint::BigUint;

pub struct Point {
    x: Option<BigUint>,
    y: Option<BigUint>,
}

impl Point {
    pub fn new(x: Option<BigUint>, y: Option<BigUint>) -> Self {
        Point { x, y }
    }
}
