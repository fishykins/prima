use crate::{core::Axis, geom::Vec2};

use super::{Float, Line2};

/// A one dimensional line. Useful for analysing only a single axis
pub struct Line1 {
    a: Float,
    b: Float,
}

impl Line1 {
    /// Generates a new Line1.
    pub fn new(a: Float, b: Float) -> Self {
        Self { a, b }
    }

    /// Checks if these two lines intersect.
    pub fn intersects(&self, other: &Self) -> bool {
        if self.a > other.b {
            return false;
        }
        if other.a > self.b {
            return false;
        }
        true
    }

    /// Gets the intersection line between these two, if any.
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if !self.intersects(other) {
            return None;
        }

        let min = if self.a > other.a { self.a } else { other.a };
        let max = if self.b < other.b { self.b } else { other.b };
        Some(Self::new(min, max))
    }

    /// Converts this into a line2, using n as the second (flat) axis.
    pub fn to_line2(self, axis: Axis, n: Float) -> Line2 {
        match axis {
            Axis::Vertical => Line2::new(Vec2::new(n, self.a), Vec2::new(n, self.b)),
            Axis::Horizontal => Line2::new(Vec2::new(self.a, n), Vec2::new(self.b, n)),
            _ => panic!("Cannot convert using this kind of axis: {:?}", axis),
        }
    }

    /// Gets a line1 from a line2, essentially disgarding a single axis.
    pub fn from_line2(other: Line2, axis: Axis) -> Self {
        match axis {
            Axis::Vertical => Self {a: other.a.y, b: other.b.y},
            Axis::Horizontal => Self {a: other.a.x, b: other.b.x},
            _ => panic!("Cannot convert using this kind of axis: {:?}", axis),
        }
    }
}
