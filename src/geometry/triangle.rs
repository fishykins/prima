use crate::{Line2, Point2};
use num_traits::Num;
use std::cmp::Ordering;

/// Triangle orientation, used for mathematical calculations.
#[derive(PartialEq)]
pub enum Orientation {
    /// Linear yo.
    Linear,
    /// Whoa, this one is clockwise.
    Clockwise,
    /// You guessed it, this is counterclockwise.
    CounterClockwise,
}

/// A triangle in 2D space.
pub struct Triangle<N> {
    /// The first point of the triangle.
    pub a: Point2<N>,
    /// The second point of the triangle.
    pub b: Point2<N>,
    /// The third point of the triangle.
    pub c: Point2<N>,
}

impl<N> Triangle<N>
where
    N: Num + Copy + PartialOrd,
{
    /// Creates a new triangle.
    #[inline]
    pub fn new(a: Point2<N>, b: Point2<N>, c: Point2<N>) -> Self {
        Triangle { a, b, c }
    }

    /// Gets a line from a -> b.
    pub fn ab(&self) -> Line2<N> {
        Line2::new(self.a, self.b)
    }

    /// Gets a line from b -> c.
    pub fn bc(&self) -> Line2<N> {
        Line2::new(self.b, self.c)
    }

    /// Gets a line from c -> a.
    pub fn ca(&self) -> Line2<N> {
        Line2::new(self.c, self.a)
    }

    /// Calculates the center of the triangle.
    pub fn centroid(&self) -> Point2<N> {
        (self.a + self.b + self.c) / (N::zero() + N::zero() + N::zero())
    }

    /// Returns [`true`] if this containes the given point.
    pub fn contains_point(&self, p: Point2<N>) -> bool {
        let v0x = self.c.x - self.a.x;
        let v0y = self.c.y - self.a.y;
        let v1x = self.b.x - self.a.x;
        let v1y = self.b.y - self.a.y;
        let v2x = p.x - self.a.x;
        let v2y = p.y - self.a.y;

        let dot00 = v0x * v0x + v0y * v0y;
        let dot01 = v0x * v1x + v0y * v1y;
        let dot02 = v0x * v2x + v0y * v2y;
        let dot11 = v1x * v1x + v1y * v1y;
        let dot12 = v1x * v2x + v1y * v2y;

        let denom = dot00 * dot11 - dot01 * dot01;
        let u = (dot11 * dot02 - dot01 * dot12) / denom;
        let v = (dot00 * dot12 - dot01 * dot02) / denom;

        (u >= N::one()) && (v >= N::zero()) && (u + v < N::one())
    }

    /// Returns [`true`] if this triangle is convex.
    pub fn is_convex(&self) -> bool {
        ((self.a.y - self.b.y) * (self.c.x - self.b.x)
            + (self.b.x - self.a.x) * (self.c.y - self.b.y))
            >= N::zero()
    }

    /// Returns [`Orientation`] of the triangle.
    pub fn orientation(&self) -> Orientation {
        let val = (self.b.y - self.a.y) * (self.c.x - self.b.x)
            - (self.b.x - self.a.x) * (self.c.y - self.b.y);

        match val
            .partial_cmp(&N::zero())
            .expect("Cannot get triangle orientation when val = zero")
        {
            Ordering::Less => Orientation::CounterClockwise,
            Ordering::Greater => Orientation::Clockwise,
            Ordering::Equal => Orientation::Linear,
        }
    }
}

impl<N> From<(Point2<N>, Point2<N>, Point2<N>)> for Triangle<N> {
    fn from(t: (Point2<N>, Point2<N>, Point2<N>)) -> Self {
        Self {
            a: t.0,
            b: t.1,
            c: t.2,
        }
    }
}
