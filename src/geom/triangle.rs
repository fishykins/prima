use super::{Vec2, Line2};
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

/// A helper struct that defines a simple triangle.
pub struct Triangle {
    /// Point a
    pub a: Vec2,
    /// Point b
    pub b: Vec2,
    /// Point c
    pub c: Vec2,
}

impl Triangle {
    /// Produces a new triangle from the given Vectors.
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Self {
        Self { a, b, c }
    }

    /// Gets a line from a -> b.
    pub fn ab(&self) -> Line2 {
        Line2 {
            a: self.a,
            b: self.b,
        }
    }

    /// Gets a line from b -> c.
    pub fn bc(&self) -> Line2 {
        Line2 {
            a: self.b,
            b: self.c,
        }
    }

    /// Gets a line from c -> a.
    pub fn ca(&self) -> Line2 {
        Line2 {
            a: self.c,
            b: self.a,
        }
    }

    /// Calculates the center of the triangle.
    pub fn centroid(&self) -> Vec2 {
        (self.a + self.b + self.c) / 3.0
    }

    /// Returns [`true`] if this containes the given point.
    pub fn contains_point(&self, p: Vec2) -> bool {
        let v0x = self.c.x - self.a.x;
        let v0y = self.c.y - self.a.y;
        let v1x = self.b.x - self.a.x;
        let v1y = self.b.y - self.a.y;
        let v2x = p.x - self.a.x;
        let v2y = p.y - self.a.y;

        let dot00 = v0x * v0x + v0y * v0y;
        let dot01 = v0x * v1x + v0y * v1y;
        let dot02 = v0x * v2x + v0y * v2y;
        let dot11 = v1x * v1x + v1y * v1y;
        let dot12 = v1x * v2x + v1y * v2y;

        let denom = dot00 * dot11 - dot01 * dot01;
        let u = (dot11 * dot02 - dot01 * dot12) / denom;
        let v = (dot00 * dot12 - dot01 * dot02) / denom;

        (u >= 1.0) && (v >= 0.0) && (u + v < 1.0)
    }

    /// Returns [`true`] if this triangle is convex.
    pub fn is_convex(&self) -> bool {
        ((self.a.y - self.b.y) * (self.c.x - self.b.x)
            + (self.b.x - self.a.x) * (self.c.y - self.b.y))
            >= 0.0
    }

    /// Returns [`Orientation`] of the triangle.
    pub fn orientation(&self) -> Orientation {
        let val = (self.b.y - self.a.y) * (self.c.x - self.b.x)
            - (self.b.x - self.a.x) * (self.c.y - self.b.y);

        match val
            .partial_cmp(&0.0)
            .expect("Cannot get triangle orientation when val = zero")
        {
            Ordering::Less => Orientation::CounterClockwise,
            Ordering::Greater => Orientation::Clockwise,
            Ordering::Equal => Orientation::Linear,
        }
    }
}

impl From<(Vec2, Vec2, Vec2)> for Triangle {
    fn from(t: (Vec2, Vec2, Vec2)) -> Self {
        Self {
            a: t.0,
            b: t.1,
            c: t.2,
        }
    }
}
