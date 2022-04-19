use num_traits::Num;

use crate::{geometry::Vector2, Point2};

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

/// A simple enum to represent the possible axis of caresian space.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AxisValue<N> where N:  num_traits::Num {
    /// The x axis.
    X(N),
    /// The y axis.
    Y(N),
    /// The z axis.
    Z(N),
    /// The x and y axis.
    XY(N, N),
    /// The x and z axis.
    XZ(N, N),
    /// The y and z axis.
    YZ(N, N),
    /// The x, y and z axis.
    XYZ(N, N, N),
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

impl<N> From<Vector2<N>> for AxisValue<N> where N: Num {
    fn from(vector: Vector2<N>) -> Self {
        Self::XY(vector.x, vector.y)
    }
}

impl<N> From<Point2<N>> for AxisValue<N> where N: Num {
    fn from(point: Point2<N>) -> Self {
        Self::XY(point.x, point.y)
    }
}

impl<N> Into<Vector2<N>> for AxisValue<N> where N: Num + Copy {
    fn into(self) -> Vector2<N> {
        match self {
            AxisValue::X(x) => Vector2::new(x, N::zero()),
            AxisValue::Y(y) => Vector2::new(N::zero(), y),
            AxisValue::Z(_) => Vector2::new(N::zero(), N::zero()),
            AxisValue::XY(x, y) => Vector2::new(x, y),
            AxisValue::XZ(x, z) => Vector2::new(x, z),
            AxisValue::YZ(y, _) => Vector2::new(N::zero(), y),
            AxisValue::XYZ(x, y, _) => Vector2::new(x, y),
            AxisValue::None => Vector2::new(N::zero(), N::zero()),
        }
    }
}

impl<N> Into<Point2<N>> for AxisValue<N> where N: Num + Copy {
    fn into(self) -> Point2<N> {
        match self {
            AxisValue::X(x) => Point2::new(x, N::zero()),
            AxisValue::Y(y) => Point2::new(N::zero(), y),
            AxisValue::Z(_) => Point2::new(N::zero(), N::zero()),
            AxisValue::XY(x, y) => Point2::new(x, y),
            AxisValue::XZ(x, z) => Point2::new(x, z),
            AxisValue::YZ(y, _) => Point2::new(N::zero(), y),
            AxisValue::XYZ(x, y, _) => Point2::new(x, y),
            AxisValue::None => Point2::new(N::zero(), N::zero()),
        }
    }
}

impl<N> From<Vec<N>> for AxisValue<N> where N: Num + Copy {
    fn from(vector: Vec<N>) -> Self {
        match vector.len() {
            0 => Self::None,
            1 => Self::X(vector[0]),
            2 => Self::XY(vector[0], vector[1]),
            _ => Self::XYZ(vector[0], vector[1], vector[2]),
        }
    }
}

impl<N> PartialEq<Axis> for AxisValue<N> where N: Num {
    fn eq(&self, other: &Axis) -> bool {
        match (self, other) {
            (AxisValue::X(_), Axis::X) => true,
            (AxisValue::Y(_), Axis::Y) => true,
            (AxisValue::Z(_), Axis::Z) => true,
            (AxisValue::XY(_, _), Axis::XY) => true,
            (AxisValue::XZ(_, _), Axis::XZ) => true,
            (AxisValue::YZ(_, _), Axis::YZ) => true,
            (AxisValue::XYZ(_, _, _), Axis::XYZ) => true,
            _ => false,
        }
    }
}