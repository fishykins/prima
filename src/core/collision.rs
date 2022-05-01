use super::*;

/// Represents a collision between two shapes.
pub struct Collision<N> {
    /// The point of contact.
    pub point: Point<N>,
    /// The normal of the contact.
    pub normal: Vector<N>,
    /// The penetration depth of the contact.
    pub depth: N,
}