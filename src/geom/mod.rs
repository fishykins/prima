mod triangle;
mod polygon;
mod is_convex;
mod line;

// Locally used functions
pub(crate) use is_convex::{is_convex, is_triangle_convex};

// Organic geometric shapes
pub use triangle::Triangle;
pub use polygon::Polygon;

// Vek re-exports
pub use vek::Aabr as BoundingRect;
pub use vek::Aabb as BoundingBox;
pub use vek::LineSegment2 as Line;
pub use vek::Rect;
pub use vek::Disk;
pub use vek::Ellipsis;

// Vek extender traits
pub use line::LineExt;