use num_traits::Num;

use crate::{geometry::Vector, Point};

/// A simple enum to represent the possible axis of caresian space.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Axis {
    /// The x axis.
    X,
    /// The y axis.
    Y,
    /// The x and y axis.
    XY,
    /// The x and z axis.
    XZ,
    /// The y and z axis.
    YZ,
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
    /// The x and y axis.
    XY(N, N),
    /// No axis.
    None,
}

impl<N> From<Vector<N>> for Axis where N: Num {
    fn from(vector: Vector<N>) -> Self {
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

impl<N> From<Vector<N>> for AxisValue<N> where N: Num {
    fn from(vector: Vector<N>) -> Self {
        Self::XY(vector.x, vector.y)
    }
}

impl<N> From<Point<N>> for AxisValue<N> where N: Num {
    fn from(point: Point<N>) -> Self {
        Self::XY(point.x, point.y)
    }
}

impl<N> Into<Vector<N>> for AxisValue<N> where N: Num + Copy {
    fn into(self) -> Vector<N> {
        match self {
            AxisValue::X(x) => Vector::new(x, N::zero()),
            AxisValue::Y(y) => Vector::new(N::zero(), y),
            AxisValue::XY(x, y) => Vector::new(x, y),
            AxisValue::None => Vector::new(N::zero(), N::zero()),
        }
    }
}

impl<N> Into<Point<N>> for AxisValue<N> where N: Num + Copy {
    fn into(self) -> Point<N> {
        match self {
            AxisValue::X(x) => Point::new(x, N::zero()),
            AxisValue::Y(y) => Point::new(N::zero(), y),
            AxisValue::XY(x, y) => Point::new(x, y),
            AxisValue::None => Point::new(N::zero(), N::zero()),
        }
    }
}

impl<N> From<Vec<N>> for AxisValue<N> where N: Num + Copy {
    fn from(vector: Vec<N>) -> Self {
        match vector.len() {
            1 => Self::X(vector[0]),
            2 => Self::XY(vector[0], vector[1]),
            _ => Self::None,
        }
    }
}

impl<N> PartialEq<Axis> for AxisValue<N> where N: Num {
    fn eq(&self, other: &Axis) -> bool {
        match (self, other) {
            (AxisValue::X(_), Axis::X) => true,
            (AxisValue::Y(_), Axis::Y) => true,
            (AxisValue::XY(_, _), Axis::XY) => true,
            _ => false,
        }
    }
}