use elliptic_curve::EllipticCurve;
use num_bigint::BigUint;
use point::Point;

mod elliptic_curve;
mod point;

fn main() {
    let a = BigUint::from(2u32);
    let b = BigUint::from(3u32);
    let p = BigUint::from(97u32);

    let curve = EllipticCurve::new(a, b, p);

    let x1 = BigUint::from(3u32);
    let y1 = BigUint::from(6u32);
    let p1 = Point::new(Some(x1), Some(y1));

    let x2 = BigUint::from(10u32);
    let y2 = BigUint::from(22u32);
    let p2 = Point::new(Some(x2), Some(y2));

    let p3 = curve.add_points(&p1, &p2);
    println!("P3: {:?}", p3);

    let scalar = BigUint::from(2u32);
    let p4 = curve.scalar_multiplication(&scalar, &p1);

    println!("P4: {:?}", p4);
}
