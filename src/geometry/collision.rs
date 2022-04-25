use crate::{Vector, Point};

/// The result of a collision between two shapes. While physics is outside the scope of this library,
/// the idea of a collision is fairly agnostic and can be used for many different purposes.
#[derive(Debug, Clone, Default)]
pub struct Collision<N = f32> {
    /// The amount of penetration between the two shapes.
    pub penetration: N,
    /// The direction of the collision.
    pub normal: Vector<N>,
    /// The point of contact between the two shapes.
    pub contact: Point<N>,
}