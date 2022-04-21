use num_traits::{Float, Signed};

use super::Axis;
use crate::{geometry::Vector, Rotation, PrimaFloat};

/// Represents the six possible directions of movement.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    /// Left
    Left,
    /// Right
    Right,
    /// Up
    Up,
    /// Down
    Down,
}

impl Direction {
    /// Returns the opposite direction.
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    /// Returns the axis that this direction sits on.
    pub fn axis(&self) -> Axis {
        match self {
            Direction::Left => Axis::X,
            Direction::Right => Axis::X,
            Direction::Up => Axis::Y,
            Direction::Down => Axis::Y,
        }
    }
}

impl<N> Into<Vector<N>> for Direction where N: Float + Signed {
    fn into(self) -> Vector<N> {
        match self {
            Direction::Left => Vector::new(N::zero() - N::one(), N::zero()),
            Direction::Right => Vector::new(N::one(), N::zero()),
            Direction::Up => Vector::new(N::zero(), N::one()),
            Direction::Down => Vector::new(N::zero(), N::zero() - N::one()),
        }
    }
}

impl Into<Axis> for Direction {
    fn into(self) -> Axis {
        match self {
            Direction::Left => Axis::X,
            Direction::Right => Axis::X,
            Direction::Up => Axis::Y,
            Direction::Down => Axis::Y,
        }
    }
}

impl<N> Into<Rotation<N>> for Direction where N: PrimaFloat {
    fn into(self) -> Rotation<N> {
        match self {
            Direction::Left => Rotation::from_radians(N::from_f32(1.5).unwrap()),
            Direction::Right => Rotation::from_radians(N::from_f32(0.5).unwrap()),
            Direction::Up => Rotation::from_radians(N::zero()),
            Direction::Down => Rotation::from_radians(N::one()),
        }
    }
}