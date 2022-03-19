use std::marker::PhantomData;

use num_traits::Num;

use crate::{Collide, Point};

use super::Point2;

/// Alias for a 2D point.
pub type Line2<N> = Line<N>;

/// A line from point to point.
#[derive(Clone, Debug, PartialEq)]
pub struct Line<N, P = Point2<N>>
where
    N: Num,
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
    N: Num,
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

impl<N, P> Collide for Line<N, P>
where
    N: Num + PartialOrd + Copy,
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