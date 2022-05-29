use prima::prelude::*;
use assert_approx_eq::assert_approx_eq;


const IOTA: f32 = 0.001;

#[test]
fn vector_test() {
    let v = Vector::new(1.0f32, 1.0f32);
    let r = v.as_rotation();
    assert_eq!(r.as_degrees(), 45.0f32);


    let a1 = Angle::new(0.25f32);
    let a2 = Angle::new(1.75f32);
    let r = a1.rotation_to(&a2);
    assert_eq!(r.as_degrees(), -90.0f32);

    let r = Rotation::new(-0.5f32);
    let v = r.as_vector();
    assert_approx_eq!(v.x, -1.0f32, IOTA);
    assert_approx_eq!(v.y, 0.0f32, IOTA);

    let v1 = Vector::new(10.0f32, 0.0f32);
    let v2 = Vector::new(0.0f32, 1.0f32);
    let r = v1.angle_of_difference(&v2);
    assert_eq!(r.as_degrees(), 90.0f32);

    let a3 = a1.lerp(&a2, 0.5f32);
    assert_eq!(a3.as_degrees(), 0.0f32);
}