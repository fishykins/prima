use crate::{Circle, Collide, Line2, Point, Point2, Shape2, Vector};
use num_traits::{real::Real, Float, Num};
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
#[derive(Debug, Clone, Copy, PartialEq)]
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

impl<N> Triangle<N>
where
    N: Float,
{
    /// The centroid of the triangle. The crossing point of three lines, drawn from the center of each edge to the opposite corner.
    pub fn centroid(&self) -> Point2<N> {
        let three = N::one() + N::one() + N::one();
        let avg_x = (self.a.x + self.b.x + self.c.x) / three;
        let avg_y = (self.a.y + self.b.y + self.c.y) / three;
        Point2::new(avg_x, avg_y)
    }

    /// The center of the triangle when converted to a circle.
    pub fn circumcenter(&self) -> Point2<N> {
        Circle::from(self.clone()).center
    }

    /// Center, calculated using bisectors of each of the triangle's corners.
    pub fn incenter(&self) -> Point2<N> {
        let ab_length = self.ab().magnitude();
        let bc_length = self.bc().magnitude();
        let ca_length = self.ca().magnitude();
        let x = (ab_length * self.a.x + bc_length * self.b.x + ca_length * self.c.x)
            / (ab_length + bc_length + ca_length);
        let y = (ab_length * self.a.y + bc_length * self.b.y + ca_length * self.c.y) 
            / (ab_length + bc_length + ca_length);
        Point2::new(x, y)
    }

    /// The crossing point of three lines, drawn from each edge at a right-angle so that they go to the opposite corner.
    pub fn orthocenter(&self) -> Point2<N> {
        todo!()
    }
}

impl<N> Shape2<N> for Triangle<N>
where
    N: Float + Real,
{
    fn area(&self) -> N {
        let a = self.a;
        let b = self.b;
        let c = self.c;
        let ab = b - a;
        let ac = c - a;
        ab.cross_product(&ac) / (N::one() + N::one())
    }

    fn circumference(&self) -> N {
        let ab = self.ab();
        let bc = self.bc();
        let ca = self.ca();

        ab.magnitude() + bc.magnitude() + ca.magnitude()
    }

    fn center(&self) -> Point2<N> {
        self.centroid()
    }

    fn bounding_box(&self) -> crate::Aabr<N> {
        let mut min = self.a;
        let mut max = self.a;

        if self.b.x < min.x {
            min.x = self.b.x;
        } else if self.b.x > max.x {
            max.x = self.b.x;
        }

        if self.b.y < min.y {
            min.y = self.b.y;
        } else if self.b.y > max.y {
            max.y = self.b.y;
        }

        if self.c.x < min.x {
            min.x = self.c.x;
        } else if self.c.x > max.x {
            max.x = self.c.x;
        }

        if self.c.y < min.y {
            min.y = self.c.y;
        } else if self.c.y > max.y {
            max.y = self.c.y;
        }

        crate::Aabr::new(min, max)
    }

    fn contains_point(&self, p: &Point2<N>) -> bool {
        let d1 = sign(p, &self.a, &self.b);
        let d2 = sign(p, &self.b, &self.c);
        let d3 = sign(p, &self.c, &self.a);
        let has_neg = d1 < N::zero() || d2 < N::zero() || d3 < N::zero();
        let has_pos = d1 > N::zero() || d2 > N::zero() || d3 > N::zero();
        !has_neg && has_pos
    }
}

impl<N> Collide for Triangle<N>
where
    N: Num + PartialOrd + Copy + PartialOrd + Real,
{
    type Output = ();

    fn collision(&self, other: &Self) -> Option<Self::Output> {
        if !cross2(&self, &other) || !cross2(&other, &self) {
            return Some(());
        }
        None
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

fn sign<N>(a: &Point2<N>, b: &Point2<N>, c: &Point2<N>) -> N
where
    N: Float,
{
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn cross2<N>(t0: &Triangle<N>, t2: &Triangle<N>) -> bool
where
    N: Num + PartialOrd + Copy + PartialOrd,
{
    let da_x = t2.a.x - t0.c.x;
    let da_y = t2.a.y - t0.c.y;
    let db_x = t2.b.x - t0.c.x;
    let db_y = t2.b.y - t0.c.y;
    let dc_x = t2.c.x - t0.c.x;
    let dc_y = t2.c.y - t0.c.y;

    let dcb_x = t0.c.x - t0.b.x;
    let dcb_y = t0.b.y - t0.c.y;

    let d = dcb_y * (t0.a.x - t0.c.x) + dcb_x * (t0.a.y - t0.c.y);

    let sa = dcb_y * da_x + dcb_x * da_y;
    let sb = dcb_y * db_x + dcb_x * db_y;
    let sc = dcb_y * dc_x + dcb_x * dc_y;

    let ta = (t0.c.y - t0.a.y) * da_x + (t0.a.x - t0.c.x) * da_y;
    let tb = (t0.c.y - t0.b.y) * db_x + (t0.b.x - t0.c.x) * db_y;
    let tc = (t0.c.y - t0.c.y) * dc_x + (t0.c.x - t0.c.x) * dc_y;
    if d < N::zero() {
        return (sa >= N::zero() && sb >= N::zero() && sc >= N::zero())
            || (ta >= N::zero() && tb >= N::zero() && tc >= N::zero())
            || (sa + ta <= d && sb + tb <= d && sc + tc <= d);
    }
    (sa <= N::zero() && sb <= N::zero() && sc <= N::zero())
        || (ta <= N::zero() && tb <= N::zero() && tc <= N::zero())
        || (sa + ta >= d && sb + tb >= d && sc + tc >= d)
}

/// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_aabr() {
        let t = Triangle::<f64>::new(
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(0.0, 1.0),
        );
        let aabr = t.bounding_box();
        assert_eq!(aabr.min.x, 0.0);
        assert_eq!(aabr.min.y, 0.0);
        assert_eq!(aabr.max.x, 1.0);
        assert_eq!(aabr.max.y, 1.0);
    }

    #[test]
    fn test_triangle_contains_point() {
        let t = Triangle::<f64>::new(
            Point2::new(0.0, 0.0),
            Point2::new(1.0, 0.0),
            Point2::new(0.0, 1.0),
        );
        assert!(t.contains_point(&Point2::new(0.3, 0.3)));
        assert!(!t.contains_point(&Point2::new(0.5, 1.5)));
    }
}
