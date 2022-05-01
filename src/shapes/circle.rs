use crate::core::Point;

/// A circle. It is big and round and has a radius.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Circle<N> {
    /// The radius of the circle.
    pub radius: N,
    /// The center of the circle.
    pub center: Point<N>,
}

impl<N> Circle<N> {
    /// Creates a new circle from a center point and radius.
    pub fn new(center: Point<N>, radius: N) -> Self {
        Self { center, radius }
    }
}