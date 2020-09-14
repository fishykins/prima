use num::Num;
use vek::Vec2;

#[inline]
pub fn is_convex<T>(points: Vec<Vec2<T>>) -> bool
where
    T: Copy + Num + PartialOrd,
{
    let n = points.len();
    if n < 3 {
        true
    } else {
        let mut i = 0;
        let l = n - 2;

        while i < l {
            if !is_triangle_convex(points[i], points[i + 1], points[i + 2]) {
                return false;
            } else {
                i += 3;
            }
        }

        if !is_triangle_convex(points[l], points[l + 1], points[0]) {
            return false;
        }
        if !is_triangle_convex(points[l + 1], points[0], points[1]) {
            return false;
        }

        true
    }
}

#[inline]
pub fn is_triangle_convex<T>(a: Vec2<T>, b: Vec2<T>, c: Vec2<T>) -> bool
where
    T: Copy + Num + PartialOrd,
{
    ((a.y - b.y) * (c.x - b.x) + (b.x - a.x) * (c.y - b.y)) >= T::zero()
}

// #[test]
// fn test_is_convex() {
//     let convex_points = [[1, -1], [1, 1], [-1, 1], [1, -1], [-1, 1], [-1, -1]];
//     assert!(is_convex(&convex_points));

//     let concave_points = [[1, -1], [-1, 1], [1, 1], [1, -1], [-1, -1], [-1, 1]];
//     assert!(!is_convex(&concave_points));
// }