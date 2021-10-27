mod line2;
mod polygon;
mod rect;
mod triangle;
mod circle;

pub use circle::Circle;
pub use line2::Line2;
pub use polygon::Polygon;
pub use rect::Rect;
pub use triangle::{Orientation, Triangle};

/// A default Vec2 type to use.
/// TODO: Impliment crate feature for f64.
pub type Vec2 = glam::Vec2;
/// Default float type to use with Vec2.
pub type Float = f32;
/// A handy value of PI that matches the Float type.
pub const PI: Float = std::f32::consts::PI;