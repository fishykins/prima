use super::{Triangle, Vec2, Float};

/// A simple circle, defined by a center and radius.
/// # Examples
///
/// ```
/// let circle = Circle::new(Vec2::new(0.0, 0.0), 32.0);
/// ```
pub struct Circle {
    /// The middle of the circle.
    pub center: Vec2,
    /// Radius of the circle.
    pub radius: Float,
}

impl Circle {
    /// Builds a circle from given center and radius.
    pub fn new(center: Vec2, radius: Float) -> Self {
        Self {
            center,
            radius,
        }
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
