use prima::prelude::*;

#[test]
fn vector_test() {
    let v = Vector::new(2.0f32, 1.0f32);
    let r = v.as_rotation();
    println!("{:?}", r.as_degrees());
}