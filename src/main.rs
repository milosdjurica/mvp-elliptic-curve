use elliptic_curve::EllipticCurve;
use num_bigint::BigUint;
use point::Point;

mod elliptic_curve;
mod point;
mod tests;

fn main() {
    let a = BigUint::from(2u32);

    let b = BigUint::from(3u32);
    let p = BigUint::from(97u32);

    let curve = EllipticCurve::new(a, b, p);

    let x1 = BigUint::from(0u32);
    let y1 = BigUint::from(10u32);
    let point1 = Point::new(Some(x1), Some(y1));

    let x2 = BigUint::from(3u32);
    let y2 = BigUint::from(91u32);
    let point2 = Point::new(Some(x2), Some(y2));

    let point3 = curve.add_points(&point1, &point2);
    println!("Point3 (add_points): {:?}", point3);

    let scalar = BigUint::from(2u32);
    let point4 = curve.scalar_multiplication(&scalar, &point1);

    println!("Point4 (scalar_multiplication): {:?}", point4);
}
