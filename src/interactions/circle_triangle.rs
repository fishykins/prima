use crate::{Circle, Distance, Point, PrimaFloat, Triangle};

impl<N> From<Triangle<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn from(triangle: Triangle<N>) -> Self {
        let minus_two = -N::one() - N::one();

        let p1 = triangle.a;
        let p2 = triangle.b;
        let p3 = triangle.c;

        let c1 = p3.x * p3.x + p3.y * p3.y - p1.x * p1.x - p1.y * p1.y;
        let c2 = p3.x * p3.x + p3.y * p3.y - p2.x * p2.x - p2.y * p2.y;
        let a1 = minus_two * (p1.x - p3.x);
        let a2 = minus_two * (p2.x - p3.x);
        let b1 = minus_two * (p1.y - p3.y);
        let b2 = minus_two * (p2.y - p3.y);

        let numer = c1 * a2 - c2 * a1;
        let denom = b1 * a2 - b2 * a1;

        if denom == N::zero() {
            panic!("Impossible to find center of circle from triangle, perhaps it is not a valid triangle?");
        }
        let y_cen = numer / denom;

        let x_cen = if a2 != N::zero() {
            (c2 - b2 * y_cen) / a2
        } else {
            (c1 - b1 * y_cen) / a1
        };

        let center = Point::new(x_cen, y_cen);
        let radius = center.distance(&p1);
        Self { center, radius }
    }
}