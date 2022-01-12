use crate::{core::Axis, geom::Vec2};

use super::{Float, Line2};

/// A one dimensional line. Useful for analysing only a single axis
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Line1 {
    a: Float,
    b: Float,
}

impl Line1 {
    /// Generates a new Line1.
    pub fn new(a: Float, b: Float) -> Self {
        Self { a, b }
    }

    /// Ensures that the line goes from lowest point to highest.
    pub fn validate(&mut self) {
        if self.a > self.b {
            std::mem::swap(&mut self.a, &mut self.b);
        }
    }

    /// Returns false is b is less than or equal to a.
    pub fn is_valid(&self) -> bool {
        self.a < self.b
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

    /// Returns true if point lies on the line. It cannot be equal to an end point.
    pub fn contains_point(&self, point: Float) -> bool {
        point > self.a && point < self.b
    }   

    /// Returns true if other line is contained. It cannot be equal to an end point.
    pub fn contains(&self, other: &Self) -> bool {
        self.contains_point(other.a) && self.contains_point(other.b)
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

    /// Subtracts the other line from this one, leaving one or two new lines.
    pub fn subtract(mut self, other: Self) -> Vec<Self> {
        self.validate();
        if !self.intersects(&other) {
            return vec![self];
        }
        if self.contains(&other) {
            return vec![Line1::new(self.a, other.a), Line1::new(other.b, self.b)];
        }
        if self.contains_point(other.a) {
            return vec![Line1::new(self.a, other.a)];
        }
        if self.contains_point(other.b) {
            return vec![Line1::new(other.b, self.b)];
        }
        return vec![self];
    }

    /// Takes a collection of lines and geometrically subtracts the given line from them.
    pub fn subtract_collection(old: Vec<Self>, other: Self) -> Vec<Self> {
        let mut new = Vec::new();
        for line in old {
            new.extend(line.subtract(other.clone()));
        }
        return new;
    }
}
