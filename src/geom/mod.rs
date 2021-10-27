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

pub type Vec2 = glam::Vec2;
pub type Float = f32;
pub const PI: Float = std::f32::consts::PI;