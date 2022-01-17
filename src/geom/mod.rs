mod circle;
mod cuboid;
mod line1;
mod line2;
mod polygon;
mod rect;
mod triangle;

pub use circle::Circle;
pub use cuboid::Cuboid;
pub use line1::Line1;
pub use line2::Line2;
pub use polygon::Polygon;
pub use rect::Rect;
pub use triangle::{Orientation, Triangle};

/// Default float type to use with Vec2.
pub type Float = f32;
/// A handy value of PI that matches the Float type.
pub const PI: Float = std::f32::consts::PI;

// Re-export glam
pub use glam::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

/// Any shape that can be represented by a 2D axis aligned bounding box.
pub trait Shape {
    /// Returns the center of this shape.
    fn center(&self) -> Vec2;
    /// Returns an axis-aligned bounding-box for this shape.
    fn bounds(&self) -> Rect;
    /// Returns the x range of the shape.
    fn x_range(&self) -> Line1;
    /// Returns the y range of the shape.
    fn y_range(&self) -> Line1;
    /// Returns true if point lies in shape.
    fn contains_point(&self, point: Vec2) -> bool;
}

/// A trait to allow for collisions between shapes.
pub trait Intersect<T, R> {
    /// Returns true if the two shapes intersect.
    fn intersects(&self, other: &T) -> bool;
    /// Returns the intersection of the two shapes.
    fn intersection(&self, other: &T) -> Option<R>;
}

/// The intersection point(s) of two shapes.
pub enum PointIntersection {
    /// A single intersection point.
    One(Vec2),
    /// Two intersection points.
    Two(Vec2, Vec2),
}

impl PointIntersection {
    /// Automatically extends from one to two points.
    pub fn add(self, p: Vec2) -> Self {
        match self {
            PointIntersection::One(v) => PointIntersection::Two(v, p),
            PointIntersection::Two(v1, v2) => PointIntersection::Two(v1, v2),
        }
    }
}