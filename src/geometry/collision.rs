use crate::Vector2;

/// The result of a collision between two shapes. While physics is outside the scope of this library,
/// the idea of a collision is fairly agnostic and can be used for many different purposes.
#[derive(Debug, Clone, Default)]
pub struct Collision<N = f32> {
    /// The amount of penetration between the two shapes.
    pub penetration: N,
    /// The direction of the collision.
    pub normal: Vector2<N>,
}