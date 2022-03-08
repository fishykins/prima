use crate::base::Point;

use super::Point2;

/// Axis-aligned bounding rectangle.
pub type Aabr<N> = BoundingBox<Point2<N>>;

/// Axis-aligned bounding thingy
pub struct BoundingBox<P> where P: Point {
    /// The minimum point of the box.
    pub min: P,
    /// The maximum point of the box.
    pub max: P,
}