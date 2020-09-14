mod triangle;
mod polygon;
mod is_convex;

pub mod line;

pub use triangle::Triangle;
pub use polygon::Polygon;
pub use is_convex::{is_convex, is_triangle_convex};