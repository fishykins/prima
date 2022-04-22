use super::Point;
use crate::{
    Aabr, Collision, Cross, Distance, FastDistance, Interact, Intersect, PrimaFloat, PrimaNum,
    Vector, Dot,
};
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
    N: PrimaNum,
{
    /// Creates a new line.
    pub fn new(start: Point<N>, end: Point<N>) -> Self {
        Line {
            start,
            end,
            phantom: PhantomData,
        }
    }

    /// Swaps the starting and ending points of the line.
    pub fn reverse(self) -> Self {
        Line {
            start: self.end,
            end: self.start,
            phantom: PhantomData,
        }
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
    /// Returns true if the line is axis aligned.
    pub fn aligned(&self) -> bool {
        self.start.aligned(&self.end)
    }

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

    fn ua_ub(&self, other: &Self) -> (N, N) {
        let x1 = self.start.x;
        let y1 = self.start.y;
        let x2 = self.end.x;
        let y2 = self.end.y;
        let x3 = other.start.x;
        let y3 = other.start.y;
        let x4 = other.end.x;
        let y4 = other.end.y;

        let ua = ((x4 - x3) * (y1 - y3) - (y4 - y3) * (x1 - x3))
            / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));

        let ub = ((x2 - x1) * (y1 - y3) - (y2 - y1) * (x1 - x3))
            / ((y4 - y3) * (x2 - x1) - (x4 - x3) * (y2 - y1));

        (ua, ub)
    }

    /// Returns the collision point of two lines, if one exists.
    /// Different to using [Interact::collision()], which produces a [Collision] rather than a point.
    pub fn collision_point(&self, other: &Self) -> Option<Point<N>> {
        let (ua, ub) = self.ua_ub(other);

        if ua >= N::zero() && ua <= N::one() && ub >= N::zero() && ub <= N::one() {
            let intersection_x = self.start.x + (ua * (self.end.x - self.start.x));
            let intersection_y = self.start.y + (ua * (self.end.y - self.start.y));
            Some(Point::new(intersection_x, intersection_y))
        } else {
            None
        }
    }

    /// Returns the closest point to p on the line.
    pub fn closest_point(&self, p: Point<N>) -> Point<N> {
        let ap: Vector<N> = (p - self.start).into();
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

impl<N> Line<N>
where
    N: PrimaFloat,
{
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
        dist_start + dist_end >= line_length - buffer
            && dist_start + dist_end <= line_length + buffer
    }
}

impl<N> Intersect for Line<N>
where
    N: PrimaFloat,
{
    fn intersecting(&self, other: &Self) -> bool {
        let (ua, ub) = self.ua_ub(other);
        ua >= N::zero() && ua <= N::one() && ub >= N::zero() && ub <= N::one()
    }
}

// NOTE: This is not a very elegant comparison to draw, and could be considered improper in the spirit of agnostic geometry. It is simply included for the sake of compleatness.
impl<N> Interact<N> for Line<N>
where
    N: PrimaFloat,
{
    fn collision(&self, other: &Self) -> Option<crate::Collision<N>> {
        if self.intersecting(other) {
            let point = self.contact_point(other)?;
            let dist_a = self.start.distance_squared(&point);
            let dist_b = self.end.distance_squared(&point);
            let dist_c = other.start.distance_squared(&point);
            let dist_d = other.end.distance_squared(&point);
            let penetration = dist_a.max(dist_b).max(dist_c).max(dist_d);
            // TODO: Figure out if this is even close to correct.
            let normal = (self.end - self.start).cross_product(&penetration).into();
            Some(Collision {
                penetration,
                normal,
            })
        } else {
            None
        }
    }

    fn nearest_extent(&self, _other: &Self) -> Point<N> {
        todo!()
    }
}
