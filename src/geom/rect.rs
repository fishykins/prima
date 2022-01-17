use super::{Circle, Float, Intersect, Line1, Line2, Shape, Vec2, PointIntersection};
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

    /// Creates a new validated rect from given min and max points.
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

    /// Returns the size of the rect.
    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    /// Returns the width of the rect.
    pub fn width(&self) -> Float {
        self.max.x - self.min.x
    }

    /// Returns the height of the rect.
    pub fn height(&self) -> Float {
        self.max.y - self.min.y
    }

    /// Returns the min x of the rect.
    pub fn left(&self) -> Float {
        self.min.x
    }

    /// Returns the max x of the rect.
    pub fn right(&self) -> Float {
        self.max.x
    }

    /// Returns the max y of the rect.
    pub fn top(&self) -> Float {
        self.max.y
    }

    /// Returns the min y of the rect.
    pub fn bottom(&self) -> Float {
        self.min.y
    }

    /// Calculates the central point of the rect.
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

    /// Returns [`true`] if this contains the given rect.
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

    /// Splits this into two new rects along the provided axis, lerped by position.
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
        let b = Rect::new(
            self.min + Vec2::new(0.0, self.height() * position),
            self.max,
        );
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

    /// Returns [`true`] if the other rect overlaps on the x axis.
    pub fn intersects_x(&self, other: &Rect) -> bool {
        !(self.min.x > other.max.x || other.min.x > self.max.x)
    }

    /// Returns [`true`] if the other rect overlaps on the y axis.
    pub fn intersects_y(&self, other: &Rect) -> bool {
        !(self.min.y > other.max.y || other.min.y > self.max.y)
    }

    /// Returns true if the two rects are touching on the x axis (not intersecting).
    pub fn touching_x(&self, other: &Rect) -> bool {
        (self.min.x == other.max.x || self.max.x == other.min.x) && self.intersects_y(other)
    }

    /// Returns true if the two rects are touching on the y axis (not intersecting).
    pub fn touching_y(&self, other: &Rect) -> bool {
        (self.min.y == other.max.y || self.max.y == other.min.y) && self.intersects_x(other)
    }

    /// Returns true if the two rects are touching, and NOT intersecting.
    pub fn touching(&self, other: &Rect) -> bool {
        (self.touching_x(other) || self.touching_y(other)) && !self.intersects(other)
    }

    /// Returns the region of contact between two rects (the line  at which they touch).
    /// Returns None if there is no contact, or if they are overlapping.
    pub fn get_touching_region(&self, other: &Rect) -> Option<Line2> {
        if self.touching_x(other) {
            let x = if self.min.x > other.min.x {
                self.min.x
            } else {
                other.min.x
            };
            return Some(
                self.y_range()
                    .intersection(&other.y_range())
                    .unwrap()
                    .into_line2(Axis::Vertical, x),
            );
        }
        if self.touching_y(other) {
            let y = if self.min.y > other.min.y {
                self.min.y
            } else {
                other.min.y
            };
            return Some(
                self.x_range()
                    .intersection(&other.x_range())
                    .unwrap()
                    .into_line2(Axis::Horizontal, y),
            );
        }
        return None;
    }
}

impl Shape for Rect {
    fn center(&self) -> Vec2 {
        self.min + self.size() / 2.0
    }

    fn bounds(&self) -> Rect {
        return self.clone();
    }

    fn x_range(&self) -> Line1 {
        return Line1::new(self.min.x, self.max.x);
    }

    fn y_range(&self) -> Line1 {
        return Line1::new(self.min.y, self.max.y);
    }

    fn contains_point(&self, p: Vec2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }
}

impl Intersect<Rect, Rect> for Rect {
    fn intersects(&self, other: &Rect) -> bool {
        self.intersects_x(other) && self.intersects_y(other)
    }

    fn intersection(&self, other: &Rect) -> Option<Rect> {
        if !self.intersects(other) {
            return None;
        }
        // Maximum of mins
        let min_x = if self.min.x > other.min.x {
            self.min.x
        } else {
            other.min.x
        };
        let min_y = if self.min.y > other.min.y {
            self.min.y
        } else {
            other.min.y
        };

        // Minimum of maxs
        let max_x = if self.max.x < other.max.x {
            self.max.x
        } else {
            other.max.x
        };
        let max_y = if self.max.y < other.max.y {
            self.max.y
        } else {
            other.max.y
        };

        Some(Rect::new(Vec2::new(min_x, min_y), Vec2::new(max_x, max_y)))
    }
}

impl Intersect<Circle, Line2> for Rect {
    fn intersects(&self, other: &Circle) -> bool {
        let closest_x = other.center.x.clamp(self.left(), self.right());
        let closest_y = other.center.y.clamp(self.bottom(), self.top());
        return Vec2::new(closest_x, closest_y).distance_squared(other.center)
            <= other.radius * other.radius;
    }

    fn intersection(&self, other: &Circle) -> Option<Line2> {
        if !self.intersects(other) {
            return None;
        }
        todo!();
    }
}

impl Intersect<Line2, PointIntersection> for Rect {
    fn intersects(&self, other: &Line2) -> bool {
        if self.contains_point(other.a) || self.contains_point(other.b) {
            return true;
        }
        let edges = self.edges();
        for edge in edges {
            if edge.intersects(other) {
                return true;
            }
        }
        false
    }

    fn intersection(&self, other: &Line2) -> Option<PointIntersection> {
        if !self.intersects(other) {
            return None;
        }
        let mut result = None;
        let edges = self.edges();
        let one_result = self.contains_point(other.a) || self.contains_point(other.b);
        for edge in edges {
            if let Some(intersection) = edge.intersection(other) {
                if result.is_none() {
                    result = Some(PointIntersection::One(intersection));
                    if one_result {
                        // As one point is inside the rect, we can return early.
                        return result;
                    }
                } else {
                    result = Some(result.unwrap().add(intersection));
                    // Only two intersections possible so return.
                    return result;
                }
            }
        }
        result
    }
}