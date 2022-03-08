use num_traits::{Float, Signed};

use super::Axis;
use crate::geometry::Vector2;

/// Represents the six possible directions of movement.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    /// Forward
    Forward,
    /// Backward
    Backward,
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
            Direction::Forward => Direction::Backward,
            Direction::Backward => Direction::Forward,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    /// Returns the axis that this direction sits on.
    pub fn axis(&self) -> Axis {
        match self {
            Direction::Forward => Axis::Z,
            Direction::Backward => Axis::Z,
            Direction::Left => Axis::X,
            Direction::Right => Axis::X,
            Direction::Up => Axis::Y,
            Direction::Down => Axis::Y,
        }
    }
}

impl<N> Into<Vector2<N>> for Direction where N: Float + Signed {
    fn into(self) -> Vector2<N> {
        match self {
            Direction::Forward => Vector2::new(N::zero(), N::one()),
            Direction::Backward => Vector2::new(N::zero(), N::zero() - N::one()),
            Direction::Left => Vector2::new(N::zero() - N::one(), N::zero()),
            Direction::Right => Vector2::new(N::one(), N::zero()),
            Direction::Up => Vector2::new(N::zero(), N::one()),
            Direction::Down => Vector2::new(N::zero(), N::zero() - N::one()),
        }
    }
}

impl Into<Axis> for Direction {
    fn into(self) -> Axis {
        match self {
            Direction::Forward => Axis::Z,
            Direction::Backward => Axis::Z,
            Direction::Left => Axis::X,
            Direction::Right => Axis::X,
            Direction::Up => Axis::Y,
            Direction::Down => Axis::Y,
        }
    }
}