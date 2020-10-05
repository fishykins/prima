mod triangle;
mod polygon;
mod line;
mod disk;

pub mod vec;

// Organic geometric shapes
pub use triangle::{Triangle, Orientation};
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
pub use disk::DiskExt;
