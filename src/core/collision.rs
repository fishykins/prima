use super::*;

/// Represents a collision between two shapes.
#[derive(Clone, Debug, Copy)]
pub struct Collision<N> {
    /// The point of contact.
    pub point: Point<N>,
    /// The normal of the contact.
    pub normal: Vector<N>,
    /// The penetration depth of the contact.
    pub depth: N,
}

impl<N> Collision<N> {
    /// Creates a new collision.
    pub fn new(point: Point<N>, normal: Vector<N>, depth: N) -> Self {
        Self { point, normal, depth }
    }
}