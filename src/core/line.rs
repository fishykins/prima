use super::{Point, Vector};
use crate::{
    nums::{PrimaFloat, PrimaNum},
    traits::{Distance, Magnitude, Nearest, Cross},
};

/// A line between two points.
pub struct Line<N> {
    /// The starting point of the line.
    pub start: Point<N>,
    /// The ending point of the line.
    pub end: Point<N>,
}

impl<N> Line<N>
where
    N: PrimaNum,
{
    /// Creates a new line.
    pub fn new(start: Point<N>, end: Point<N>) -> Line<N> {
        Line { start, end }
    }
}

impl<N> Line<N>
where
    N: PrimaFloat,
{
    /// Returns the point of collision between the two lines.
    pub fn collision(&self, other: &Self) -> Option<Point<N>> {
        let a = self.start;
        let c = other.start;
        let r = self.end - a;
        let s = other.end - c;

        let denom: N = r.cross(&s);
        if denom == N::zero() {
            return None;
        }

        let numer_a = (c - a).cross(&s);
        let numer_c = (c - a).cross(&r);

        let t = numer_a / denom;
        let u = numer_c / denom;

        if t < N::zero() || t > N::one() || u < N::zero() || u > N::one() {
            return None;
        }
        return Some(a + r * t);
    }
}

impl<N> Magnitude<N> for Line<N>
where
    N: PrimaFloat,
{
    fn magnitude_squared(&self) -> N {
        let dx = (self.end.x - self.start.x).abs();
        let dy = (self.end.y - self.start.y).abs();
        dx * dx + dy * dy
    }
}

impl<N> Distance<N, Point<N>> for Line<N>
where
    N: PrimaFloat,
{
    fn squared_distance(&self, other: &Point<N>) -> N {
        let p = self.nearest_point(other);
        let dist = p - *other;
        dist.magnitude_squared()
    }
}

impl<N> Nearest<N, Point<N>> for Line<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, other: &Point<N>) -> Point<N> {
        let ap: Vector<N> = (*other - self.start).into();
        let ab: Vector<N> = (self.end - self.start).into();

        let ab_magnitude = ab.magnitude_squared();
        let abap_product = ap.dot(&ab);
        let dist = abap_product / ab_magnitude;

        if dist < N::zero() {
            self.start
        } else if dist > N::one() {
            self.end
        } else {
            self.start + ab * dist
        }
    }
}
