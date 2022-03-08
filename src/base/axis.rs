use num_traits::Num;

use crate::geometry::Vector2;

/// A simple enum to represent the possible axis of caresian space.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Axis {
    /// The x axis.
    X,
    /// The y axis.
    Y,
    /// The z axis.
    Z,
    /// The x and y axis.
    XY,
    /// The x and z axis.
    XZ,
    /// The y and z axis.
    YZ,
    /// The x, y and z axis.
    XYZ,
    /// No axis.
    None,
}

impl<N> From<Vector2<N>> for Axis where N: Num {
    fn from(vector: Vector2<N>) -> Self {
        if vector.x != N::zero() {
            if vector.y == N::zero() {
                Self::X
            } else {
                Self::XY
            }
        } else {
            if vector.y != N::zero() {
                Self::Y
            } else {
                Self::None
            }
        }
    }
}