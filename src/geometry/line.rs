use super::Point;
use crate::{PrimaFloat, PrimaNum, Vector, Distance, Intersect, Cross, Aabr};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// A line from point to point.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Line<N>
where
    N: PrimaNum,
{
    /// The starting point of the line.
    pub start: Point<N>,
    /// The ending point of the line.
    pub end: Point<N>,
    phantom: PhantomData<N>,
}

impl<N> Line<N>
where
    N: PrimaFloat,
{
    /// Creates a new line.
    pub fn new(start: Point<N>, end: Point<N>) -> Self {
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
    pub fn contact_point(&self, other: &Self) -> Option<Point<N>> {
        let a = self.start;
        let c = other.start;
        let r: Point<N> = self.end - a;
        let s = other.end - c;

        let denom: N = r.cross_product(&s);
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

impl<N> Line<N>
where
    N: PrimaFloat,
{
    /// Bisects the line.
    pub fn bisect(self) -> Self {
        todo!()
    }

    /// Returns the squared magnitude of the line.
    pub fn magnitude_squared(&self) -> N {
        let x = self.end.x - self.start.x;
        let y = self.end.y - self.start.y;
        x * x + y * y
    }

    /// Returns the length of the line.
    pub fn magnitude(&self) -> N {
        self.magnitude_squared().sqrt()
    }

    /// Returns the unit vector of the line.
    pub fn normalize(&self) -> Vector<N> {
        let v: Vector<N> = (self.end - self.start).into();
        v.normalize()
    }

    /// Returns the bounding box of the line.
    pub fn bounding_box(&self) -> Aabr<N> {
        let x = self.start.x.min(self.end.x);
        let y = self.start.y.min(self.end.y);
        let w = self.start.x.max(self.end.x);
        let h = self.start.y.max(self.end.y);
        Aabr::new(Point::new(x, y), Point::new(w, h))
    }
}

impl<N> Line<N> where N: PrimaFloat {
    /// Returns the length of the line.
    pub fn length(&self) -> N {
        self.start.distance(&self.end)
    }

    /// Returns true if the point lies on the line.
    pub fn contains_point(&self, point: Point<N>) -> bool {
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