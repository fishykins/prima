use crate::core::{Point, Extent};

/// An axis-aligned bounding rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Aabr<N> {
    /// The center point of this rectangle.
    pub center: Point<N>,
    /// The extent of this rectangle.
    pub extent: Extent<N>,
}