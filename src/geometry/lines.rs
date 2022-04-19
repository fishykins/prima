use super::Point2;
use crate::{Point, PrimaFloat, PrimaNum, Vector, Distance, Intersect};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// Alias for a 2D point.
pub type Line2<N = super::DefaultFloat> = Line<N>;

/// A line from point to point.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Line<N, P = Point2<N>>
where
    N: PrimaNum,
    P: Point<N>,
{
    /// The starting point of the line.
    pub start: P,
    /// The ending point of the line.
    pub end: P,
    phantom: PhantomData<N>,
}

impl<N, P> Line<N, P>
where
    P: Point<N>,
    N: PrimaNum,
{
    /// Creates a new line.
    pub fn new(start: P, end: P) -> Self {
        Line {
            start,
            end,
            phantom: PhantomData,
        }
    }

    /// Returns true if the line is axis aligned.
    pub fn aligned(&self) -> bool {
        self.start.aligned(&self.end)
    }

    /// Returns the point of collision between the line and the given line.
    pub fn contact_point(&self, other: &Self) -> Option<P> {
        let a = self.start;
        let c = other.start;
        let r: P = self.end - a;
        let s = other.end - c;

        let denom = r.cross_product(&s);
        if denom == N::zero() {
            return None;
        }

        let numer_a = (c - a).cross_product(&s);
        let numer_c = (c - a).cross_product(&r);

        let t = numer_a / denom;
        let u = numer_c / denom;

        if t < N::zero() || t > N::one() || u < N::zero() || u > N::one() {
            return None;
        }
        return Some(a + r * t);
    }
}

impl<N, P> Line<N, P>
where
    N: PrimaFloat,
    P: Point<N>,
{
    /// Bisects the line.
    pub fn bisect(self) -> Self {
        todo!()
    }
}

impl<N> Vector for Line<N>
where
    N: PrimaFloat,
{
    type Output = N;

    fn magnitude_squared(&self) -> Self::Output {
        let x = self.end.x - self.start.x;
        let y = self.end.y - self.start.y;
        x * x + y * y
    }

    fn magnitude(&self) -> Self::Output {
        self.magnitude_squared().sqrt()
    }
}

impl<N> Line2<N> where N: PrimaFloat {
    /// Returns the length of the line.
    pub fn length(&self) -> N {
        self.start.distance(&self.end)
    }

    /// Returns true if the point lies on the line.
    pub fn contains_point(&self, point: Point2<N>) -> bool {
        let line_length = self.length();
        let dist_start = self.start.distance(&point);
        let dist_end = self.end.distance(&point);
        let buffer = N::from_f32(0.01).unwrap();
        dist_start + dist_end >= line_length - buffer && dist_start + dist_end <= line_length + buffer
    }
}

impl<N> Intersect for Line<N>
where
    N: PrimaFloat,
{
    fn intersecting(&self, other: &Self) -> bool {
        self.contact_point(other).is_some()
    }
}