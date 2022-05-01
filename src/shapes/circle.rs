use crate::core::Point;

/// A circle. It is big and round and has a radius.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Circle<N> {
    /// The radius of the circle.
    pub radius: N,
    /// The center of the circle.
    pub center: Point<N>,
}
