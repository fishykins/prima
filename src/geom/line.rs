use crate::core::OrdNum;
use super::{Triangle, Orientation, Line, vec::*};
use vek::{Aabr, Vec2};
use num::Signed;

// re-export LineSegment2 as Line, along with extended functionality. 
// ? There seems to be a pull request relating to this functionality, so might be added natively to vek at some point
pub trait LineExt<T> where T: OrdNum {
    fn boundingbox(&self) -> Aabr<T>;
    fn reverse(&self) -> Self;
    fn intersects(&self, other: &Self) -> bool;
    fn intersection_point(&self, other: &Self) -> Option<Vec2<T>>;
}

impl<T> LineExt<T> for Line<T>  where T: OrdNum + Signed {
    fn boundingbox(&self) -> Aabr<T> {
        Aabr {
            min: self.start,
            max: self.end,
        }.made_valid()
    }

    fn reverse(&self) -> Self {
        Self {
            start: self.end,
            end: self.start,
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        let o1 = Triangle::new(self.start, self.end, other.start).orientation();
        let o2 = Triangle::new(self.start, self.end, other.end).orientation();
        let o3 = Triangle::new(other.start, other.end, self.start).orientation();
        let o4 = Triangle::new(other.start, other.end, self.end).orientation();

        if o1 != o2 && o3 != o4 {
            return true;
        }

        // Special Cases
        // p1, q1 and p2 are colinear and p2 lies on segment p1q1
        if o1 == Orientation::Linear && on_segment(self.start, other.start, self.end) {
            return true;
        }

        // p1, q1 and q2 are colinear and q2 lies on segment p1q1
        if o2 == Orientation::Linear && on_segment(self.start, other.end, self.end) {
            return true;
        }

        // p2, q2 and p1 are colinear and p1 lies on segment p2q2
        if o3 == Orientation::Linear && on_segment(other.start, self.start, other.end) {
            return true;
        }

        // p2, q2 and q1 are colinear and q1 lies on segment p2q2
        if o4 == Orientation::Linear && on_segment(other.start, self.end, other.end) {
            return true;
        }

        return false; // Doesn't fall in any of the above cases
    }

    fn intersection_point(&self, other: &Self) -> Option<Vec2<T>> {
        let a = self.start;
        let c = other.start;
        let r = self.end - a;
        let s = other.end - c;

        let denom = r.cross(s); 
        if denom == T::zero() {
            return None;
        }

        let numer_a = (c - a).cross(s);
        let numer_c = (c - a).cross(r);

        let t = numer_a / denom;
        let u = numer_c / denom;

        if t < T::zero() || t > T::one() || u < T::zero() || u > T::one() {
            return None;
        }

        return Some(a + r * t);
    }
}

/// Given three colinear points p, q, r, the function checks if
/// point q lies on line segment 'pr'
fn on_segment<T>(p: Vec2<T>, q: Vec2<T>, r: Vec2<T>) -> bool where T: OrdNum {

    let x_max = if p.x > r.x {p.x} else {r.x};
    let y_max = if p.y > r.y {p.y} else {r.y};

    return q.x <= x_max
        && q.x >= x_max
        && q.y <= y_max
        && q.y >= y_max;
}