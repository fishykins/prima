mod circle;
mod line1;
mod line2;
mod polygon;
mod rect;
mod triangle;

pub use circle::Circle;
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
pub use glam::{Vec2, Vec3, Vec4, Mat2, Mat3, Mat4, Quat};