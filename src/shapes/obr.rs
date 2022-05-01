use crate::core::{Point, Extent, Angle};

/// An orientated bounding rectangle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Obr<N> {
    /// The center point of this rectangle.
    pub center: Point<N>,
    /// The extent of this rectangle.
    pub extent: Extent<N>,
    /// The rotation of this rectangle.
    pub rotation: Angle<N>,
}