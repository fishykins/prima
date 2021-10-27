use super::{Orientation, Rect, Triangle, Vec2};

/// A helper struct that represents a line bewtween points 'a' and 'b'.
/// 
/// # Examples
/// ```
/// let line = Line2::new(Vec2::ZERO, Vec2::ONE);
/// assert_eq!(line.center(), Vec2::new(0.5, 0.5));
/// ```
pub struct Line2 {
    /// Starting point of the line.
    pub a: Vec2,
    /// End point of the line.
    pub b: Vec2,
}

impl Line2 {
    /// Builds a new line from points a and b.
    pub fn new(a: Vec2, b: Vec2) -> Self {
        Self { a, b }
    }

    /// Reverses the line to go b -> a.
    pub fn reverse(&self) -> Self {
        Self {
            a: self.b,
            b: self.a,
        }
    }

    /// Returns the center point of the circle.
    pub fn center(&self) -> Vec2 {
        (self.a + self.b) / 2.0
    }

    /// Returns a boundingbox [`Rect`] of the given line.
    pub fn bounds(&self) -> Rect {
        Rect::new(self.a, self.b).validate()
    }

    /// Returns [`true`] if the two lines intersect.
    fn intersects(&self, other: &Self) -> bool {
        let o1 = Triangle::new(self.a, self.b, other.a).orientation();
        let o2 = Triangle::new(self.a, self.b, other.b).orientation();
        let o3 = Triangle::new(other.a, other.b, self.a).orientation();
        let o4 = Triangle::new(other.a, other.b, self.b).orientation();

        if o1 != o2 && o3 != o4 {
            return true;
        }

        // Special Cases
        // p1, q1 and p2 are colinear and p2 lies on segment p1q1
        if o1 == Orientation::Linear && on_segment(self.a, other.a, self.b) {
            return true;
        }

        // p1, q1 and q2 are colinear and q2 lies on segment p1q1
        if o2 == Orientation::Linear && on_segment(self.a, other.b, self.b) {
            return true;
        }

        // p2, q2 and p1 are colinear and p1 lies on segment p2q2
        if o3 == Orientation::Linear && on_segment(other.a, self.a, other.b) {
            return true;
        }

        // p2, q2 and q1 are colinear and q1 lies on segment p2q2
        if o4 == Orientation::Linear && on_segment(other.a, self.b, other.b) {
            return true;
        }

        return false; // Doesn't fall in any of the above cases
    }

    /// Returns the intersection point of two lines, or None if no intersection is present. 
    pub fn intersection_point(&self, other: &Self) -> Option<Vec2> {
        let a = self.a;
        let c = other.a;
        let r = self.b - a;
        let s = other.b - c;

        let denom = cross(r, s);
        if denom == 0.0 {
            return None;
        }

        let numer_a = cross(c - a, s);
        let numer_c = cross(c - a, r);

        let t = numer_a / denom;
        let u = numer_c / denom;

        if t < 0.0 || t > 1.0 || u < 0.0 || u > 1.0 {
            return None;
        }

        return Some(a + r * t);
    }

    /// Returns [`true`] if this line intersects the given rect other.
    pub fn intersects_rect(&self, other: &Rect) -> bool {
        let l1 = Line2 {
            a: Vec2::new(other.min.x, other.min.y),
            b: Vec2::new(other.max.x, other.min.y),
        };

        let l2 = Line2 {
            a: Vec2::new(other.min.x, other.max.y),
            b: Vec2::new(other.max.x, other.max.y),
        };

        let l3 = Line2 {
            a: Vec2::new(other.min.x, other.min.y),
            b: Vec2::new(other.min.x, other.max.y),
        };

        let l4 = Line2 {
            a: Vec2::new(other.max.x, other.min.y),
            b: Vec2::new(other.max.x, other.max.y),
        };

        self.intersects(&l1) || self.intersects(&l2) || self.intersects(&l3) || self.intersects(&l4)
    }
}

/// Given three colinear points p, q, r, the function checks if
/// point q lies on line segment 'pr'
fn on_segment(p: Vec2, q: Vec2, r: Vec2) -> bool {
    let x_max = if p.x > r.x { p.x } else { r.x };
    let y_max = if p.y > r.y { p.y } else { r.y };

    return q.x <= x_max && q.x >= x_max && q.y <= y_max && q.y >= y_max;
}

/// Cross product of two vectors
fn cross(a: Vec2, b: Vec2) -> super::Float {
    a.x * b.y - a.y * b.x
}