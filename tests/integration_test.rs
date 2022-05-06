use assert_approx_eq::assert_approx_eq;
use prima::{core::*, shapes::*, traits::*};

const IOTA: f32 = 0.001;

#[test]
fn point_test() {
    let p: Point<f32> = Point::new(1.0, 2.0);
    assert_eq!(p.x, 1.0);
    assert_eq!(p.y, 2.0);

    let p2 = p + Vector::new(1.5, 2.2);
    assert_eq!(p2.x, 2.5);
    assert_eq!(p2.y, 4.2);

    let v = p2 - p;
    assert_approx_eq!(v.x, 1.5, IOTA);
    assert_approx_eq!(v.y, 2.2, IOTA);
}

#[test]
fn core_test() {
    // Point point
    let a: Line<f32> = Line::new(Point::new(2.0, 2.0), Point::new(5.0, 8.0));
    let p = Point::new(3.0, 4.0);
    let n = a.nearest_point(&p);
    assert!(n.on_line(&a));
    assert_approx_eq!(a.distance(&p), n.distance(&p), IOTA);

    // Line line
    let b: Line<f32> = Line::new(Point::new(3.0, 1.0), Point::new(1.0, 10.0));
    let cross = a.collision(&b).unwrap();
    assert!(cross.on_line(&a));
    assert!(cross.on_line(&b));
}

#[test]
fn line_test() {
    let a = Line::<f32>::from_point(Point::new(2.0, 2.0), Vector::new(8.0, 0.0));
    let b = Line::<f32>::from_point(Point::new(4.0, 4.0), Vector::new(4.0, 4.0));

    let p = a.nearest_point(&b);
    assert!(p.on_line(&a));
    assert_approx_eq!(p.x, 4.0, IOTA);

}

#[test]
fn circle_test() {
    // Circle circle
    let a: Circle<f32> = Circle::new(Point::new(2.0, 2.0), 3.0);
    let b: Circle<f32> = Circle::new(Point::new(10.0, 10.0), 3.5);
    let c: Circle<f32> = Circle::new(Point::new(14.0, 10.0), 1.0);
    let ds = a.squared_distance(&b);
    assert_eq!(ds, 106.75f32);
    assert!(!a.intersecting(&b));
    assert!(a.collision(&c).is_none());
    assert!(b.intersecting(&c));
}

#[test]
fn aabr_test() {

}

#[test]
fn obr_test() {
    let a: Obr<f32> = Obr::new(
        Point::new(2.4, 2.4),
        Extent::new(2.0, 2.0),
        Angle::from_degrees(45.0f32),
    );
    assert_eq!(a.rotation.as_radians(), 0.25f32);
    let v = a.vertices();
    assert_approx_eq!(v[0].x, 0.985, IOTA);
    assert_approx_eq!(v[0].y, 2.4, IOTA);
    assert_approx_eq!(v[1].x, 2.4, IOTA);
    assert_approx_eq!(v[1].y, 3.814, IOTA);
}

#[test]
fn aabr_obr_test() {
    let a: Aabr<f32> = Aabr::new(Point::new(0.0, 0.0), Point::new(2.0, 2.0));
    let mut b: Obr<f32> = Obr::new(
        Point::new(3.0, 3.0),
        Extent::new(2.0, 2.0),
        Angle::from_degrees(45.0f32),
    );
    assert!(!a.intersecting(&b));
    b.translate(&Vector::splat(-0.6));
    assert!(a.intersecting(&b));
    b.center = Point::new(3.0, -1.0);
    assert!(!a.intersecting(&b));
    b.translate(&Vector::new(-2.0, 0.0));
    assert!(a.intersecting(&b));
}
