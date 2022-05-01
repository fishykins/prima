use crate::{nums::PrimaFloat, traits::{Nearest, Magnitude, Distance}};
use super::{Point, Vector};

/// A line between two points.
pub struct Line<N> {
    /// The starting point of the line.
    pub start: Point<N>,
    /// The ending point of the line.
    pub end: Point<N>,
}

impl<N> Line<N> {
    /// Creates a new line.
    pub fn new(start: Point<N>, end: Point<N>) -> Line<N> {
        Line { start, end }
    }
}

impl<N> Magnitude<N> for Line<N> where N: PrimaFloat {
    fn magnitude_squared(&self) -> N {
        let dx = (self.end.x - self.start.x).abs();
        let dy = (self.end.y - self.start.y).abs();
        dx * dx + dy * dy
    }
}

impl<N> Distance<N, Point<N>> for Line<N> where N: PrimaFloat {
    fn squared_distance(&self, other: &Point<N>) -> N {
        let p = self.nearest_point(other);
        let dist = p - *other;
        dist.magnitude_squared()
    }
}

impl<N> Nearest<N, Point<N>> for Line<N> where N: PrimaFloat {
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