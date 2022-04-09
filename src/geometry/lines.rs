use std::marker::PhantomData;
use crate::{Collide, Point, Vector, PrimaNum, PrimaFloat};

use super::Point2;

/// Alias for a 2D point.
pub type Line2<N = super::DefaultFloat> = Line<N>;

/// A line from point to point.
#[derive(Clone, Debug, PartialEq)]
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
}

impl<N, P> Line<N, P> where N: PrimaFloat, P: Point<N> {
    /// Bisects the line.
    pub fn bisect(self) -> Self {
        todo!()
    }
}

impl<N, P> Collide for Line<N, P>
where
    N: PrimaNum,
    P: Point<N>,
{
    type Output = P;

    fn collision(&self, other: &Self) -> Option<Self::Output> {
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

impl<N> Vector for Line<N> where N: PrimaFloat {
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