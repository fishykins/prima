use std::fmt::{Error, Formatter, Display};

use super::{Float, Intersect, Line2, Shape, Triangle, Vec2, PointIntersection};

/// A simple circle, defined by a center and radius.
/// # Examples
///
/// ```
/// let circle = Circle::new(Vec2::new(0.0, 0.0), 32.0);
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Circle {
    /// The middle of the circle.
    pub center: Vec2,
    /// Radius of the circle.
    pub radius: Float,
}

impl Circle {
    /// Builds a circle from given center and radius.
    pub fn new(center: Vec2, radius: Float) -> Self {
        Self { center, radius }
    }
    /// Generates a new circle from the given triangle.
    pub fn from_triangle(triangle: Triangle) -> Option<Self> {
        let p1 = triangle.a;
        let p2 = triangle.b;
        let p3 = triangle.c;

        let c1 = p3.x * p3.x + p3.y * p3.y - p1.x * p1.x - p1.y * p1.y;
        let c2 = p3.x * p3.x + p3.y * p3.y - p2.x * p2.x - p2.y * p2.y;
        let a1 = -2.0 * (p1.x - p3.x);
        let a2 = -2.0 * (p2.x - p3.x);
        let b1 = -2.0 * (p1.y - p3.y);
        let b2 = -2.0 * (p2.y - p3.y);

        let numer = c1 * a2 - c2 * a1;
        let denom = b1 * a2 - b2 * a1;

        if denom == 0.0 {
            return None;
        }
        let y_cen = numer / denom;

        let x_cen = if a2 != 0.0 {
            (c2 - b2 * y_cen) / a2
        } else {
            (c1 - b1 * y_cen) / a1
        };

        let center = Vec2::new(x_cen, y_cen);
        let radius = center.distance(p1);
        Some(Self { center, radius })
    }
}

impl Shape for Circle {
    fn bounds(&self) -> super::Rect {
        let r = Vec2::splat(self.radius);
        super::Rect::new(self.center - r, self.center + r)
    }

    fn x_range(&self) -> super::Line1 {
        super::Line1::new(self.center.x - self.radius, self.center.x + self.radius)
    }

    fn y_range(&self) -> super::Line1 {
        super::Line1::new(self.center.y - self.radius, self.center.y + self.radius)
    }

    fn contains_point(&self, p: Vec2) -> bool {
        self.center.distance_squared(p) <= self.radius * self.radius
    }

    fn center(&self) -> Vec2 {
        self.center
    }
}

impl Intersect<Circle, PointIntersection> for Circle {
    fn intersects(&self, other: &Circle) -> bool {
        self.center.distance_squared(other.center)
            <= (self.radius + other.radius) * (self.radius + other.radius)
    }

    fn intersection(&self, _other: &Circle) -> Option<PointIntersection> {
        todo!();
    }
}

impl Intersect<Line2, PointIntersection> for Circle {
    fn intersects(&self, other: &Line2) -> bool {
        other.intersects(self)
    }

    fn intersection(&self, other: &Line2) -> Option<PointIntersection> {
        other.intersection(self)
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Circle(r: {}, center: {})", self.radius, self.center)
    }
}