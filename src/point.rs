use num_bigint::BigUint;

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub x: Option<BigUint>,
    pub y: Option<BigUint>,
}

impl Point {
    pub fn new(x: Option<BigUint>, y: Option<BigUint>) -> Self {
        Point { x, y }
    }

    pub fn is_infinity(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
}
