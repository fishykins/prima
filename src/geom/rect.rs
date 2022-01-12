use super::{Float, Line1, Vec2, Line2};
use crate::core::Axis;

/// Axis aligned rectangle
/// # Examples
///
/// ```
/// let rect = Rect::new(Vec2::new(0.0, 0.0), Vec2::new(8.0, 8.0));
/// assert_eq!(rect.width(), 8.0);
/// assert_eq!(rect.height(), 8.0);
/// ```
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Rect {
    /// The min point of the Rect.
    pub min: Vec2,
    /// The max point of the Rect.
    pub max: Vec2,
}

impl Rect {
    /// Creates a new unvalidated Rect from given min and max points.
    /// # Example
    /// ```
    /// let rect = Rect::new(Vec2::ZERO, Vec2::splat(8.0));
    /// assert!(rect.valid());
    ///
    /// let rect2 = Rect::new(Vec2::splat(8.0), Vec2::ZERO);
    /// assert!(!rect2.valid());
    /// ```
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }

    /// Creates a new validated Rect from given min and max points.
    /// # Example
    /// ```
    /// let rect = Rect::new(Vec2::ZERO, Vec2::splat(8.0));
    /// assert!(rect.valid());
    ///
    /// let rect2 = Rect::new(Vec2::splat(8.0), Vec2::ZERO);
    /// assert!(rect2.valid());
    /// ```
    pub fn new_valid(min: Vec2, max: Vec2) -> Self {
        Self { min, max }.validate()
    }

    /// Checks if the Rect has valid min and max points.
    pub fn valid(&self) -> bool {
        self.min.x < self.max.x && self.min.y < self.max.y
    }

    /// Converts the Rect into a validated version of itself.
    pub fn validate(self) -> Self {
        Self {
            min: self.min.min(self.max),
            max: self.min.max(self.max),
        }
    }

    /// Returns the width of the Rect.
    pub fn width(&self) -> Float {
        self.max.x - self.min.x
    }

    /// Returns the height of the Rect.
    pub fn height(&self) -> Float {
        self.max.y - self.min.y
    }

    /// Calculates the central point of the Rect.
    pub fn center(&self) -> Vec2 {
        (self.min + self.max) / 2.0
    }

    /// Returns all four corners of this rect.
    pub fn corners(&self) -> [Vec2; 4] {
        [
            self.min,
            self.min + Vec2::new(0.0, self.height()),
            self.max,
            self.min + Vec2::new(self.width(), 0.0),
        ]
    }

    /// Returns four lines that represent the edges of this rect.
    pub fn edges(&self) -> [Line2; 4] {
        let points = self.corners();
        [
            Line2::new(points[0], points[1]),
            Line2::new(points[1], points[2]),
            Line2::new(points[3], points[2]),
            Line2::new(points[0], points[3]),
        ]
    }

    /// Returns [`true`] if the Rect contains the point 'p'.
    pub fn contains_point(&self, p: Vec2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    /// Returns [`true`] if this contains the given Rect.
    pub fn contains_rect(&self, other: &Rect) -> bool {
        other.min.x >= self.min.x
            && other.min.x <= self.max.x
            && other.max.x >= self.min.x
            && other.max.x <= self.max.x
            && other.min.y >= self.min.y
            && other.min.y <= self.max.y
            && other.max.y >= self.min.y
            && other.max.y <= self.max.y
    }

    /// Splits this into two new Rects along the provided axis, lerped by position.
    /// # Example
    /// ```
    /// let rect = Rect::new(Vec2::ZERO, Vec2::ONE);
    /// let (a, b) = rect.split(0.5, Axis::Vertical);
    /// ```
    pub fn split(self, position: Float, axis: Axis) -> (Self, Self) {
        let p = position.clamp(0.0, 1.0);
        match axis {
            Axis::Vertical => self.split_x(p),
            Axis::Horizontal => self.split_y(p),
            Axis::Both => todo!(),
            Axis::None => panic!("Cannot split by axis None!"),
        }
    }

    /// Splits along the x axis.
    pub fn split_x(self, position: Float) -> (Self, Self) {
        let a = Rect::new(
            self.min,
            self.min + Vec2::new(self.width() * position, self.height()),
        );
        let b = Rect::new(self.min + Vec2::new(self.width() * position, 0.0), self.max);
        (a, b)
    }

    /// Splits along the y axis.
    pub fn split_y(self, position: Float) -> (Self, Self) {
        let a = Rect::new(
            self.min,
            self.min + Vec2::new(self.width(), self.height() * position),
        );
        let b = Rect::new(self.min + Vec2::new(0.0, self.height() * position), self.max);
        (a, b)
    }

    /// Splits the rect into four equal rects.
    pub fn into_quad(self) -> [Self; 4] {
        let [a, b, c, _] = self.corners();
        let m = a + Vec2::new(self.width() / 2.0, self.height() / 2.0);
        [
            Rect::new(a, m),
            Rect::new(Vec2::new(a.x, m.y), b),
            Rect::new(m, c),
            Rect::new(Vec2::new(m.x, a.y), Vec2::new(c.x, m.y)),
        ]
    }

    /// Determines if two rects have overlapping bounds.
    pub fn intersects(&self, other: &Rect) -> bool {
        self.intersects_x(other) && self.intersects_y(other)
    }

    /// Returns [`true`] if the other rect overlaps on the x axis.
    pub fn intersects_x(&self, other: &Rect) -> bool {
        !(self.min.x > other.max.x || other.min.x > self.max.x)
    }

    /// Returns [`true`] if the other rect overlaps on the y axis.
    pub fn intersects_y(&self, other: &Rect) -> bool {
        !(self.min.y > other.max.y || other.min.y > self.max.y)
    }

    /// Returns the rect that contains the intersection of the two rects,
    /// or none if no intersection exists.
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        if !self.intersects(other) {
            return None;
        }
        // Maximum of mins
        let min_x = if self.min.x > other.min.x {self.min.x} else {other.min.x};
        let min_y = if self.min.y > other.min.y {self.min.y} else {other.min.y};

        // Minimum of maxs
        let max_x = if self.max.x < other.max.x {self.max.x} else {other.max.x};
        let max_y = if self.max.y < other.max.y {self.max.y} else {other.max.y};
        
        Some(Rect::new(Vec2::new(min_x, min_y), Vec2::new(max_x, max_y)))
    }

    /// Returns the intersection on the x axis, if there is any.
    pub fn intersection_x(&self, other: &Rect) -> Option<Line1> {
        if !self.intersects_x(other) {
            return None;
        }
        let min = if self.min.x > other.min.x {self.min.x} else {other.min.x};
        let max = if self.max.x < other.max.x {self.max.x} else {other.max.x};
        Some(Line1::new(min, max))
    }

    /// Returns the intersection on the y axis, if there is any.
    pub fn intersection_y(&self, other: &Rect) -> Option<Line1> {
        if !self.intersects_y(other) {
            return None;
        }
        let min = if self.min.y > other.min.y {self.min.y} else {other.min.y};
        let max = if self.max.y < other.max.y {self.max.y} else {other.max.y};
        Some(Line1::new(min, max))
    }

    /// Returns true if the two [Rects] are touching on the x axis (not intersecting).
    pub fn touching_x(&self, other: &Rect) -> bool {
        (self.min.x == other.max.x || self.max.x == other.min.x) && self.intersects_y(other)
    }

    /// Returns true if the two [Rects] are touching on the y axis (not intersecting).
    pub fn touching_y(&self, other: &Rect) -> bool {
        (self.min.y == other.max.y || self.max.y == other.min.y) && self.intersects_x(other)
    }

    /// Returns true if the two [Rects] are touching, and NOT intersecting.
    pub fn touching(&self, other: &Rect) -> bool {
        (self.touching_x(other) || self.touching_y(other)) && !self.intersects(other)
    }

    /// Returns the region of contact between two [Rects] (the line  at which they touch).
    /// Returns None if there is no contact, or if they are overlapping.
    pub fn get_touching_region(&self, other: &Rect) -> Option<Line2> {
        if self.touching_x(other) {
            let x = if self.min.x > other.min.x {self.min.x} else {other.min.x};
            return Some(self.intersection_y(other).unwrap().into_line2(Axis::Vertical, x));
        }
        if self.touching_y(other) {
            let y = if self.min.y > other.min.y {self.min.y} else {other.min.y};
            return Some(self.intersection_x(other).unwrap().into_line2(Axis::Horizontal, y));
        }
        return None;
    }
}
