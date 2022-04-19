use serde::{Deserialize, Serialize};

use crate::{xy_ops_impl, common::{Dot, Vector}, PrimaFloat, PrimaNum, abstracts::Direction};
use super::Point2;

/// A base struct for 2D points/vectors.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Vector2<N = super::DefaultFloat> {
    /// The X magnitude.
    pub x: N,
    /// The Y magnitude.
    pub y: N,
}

xy_ops_impl!(Vector2);

impl<N> Dot for Vector2<N> where N: PrimaNum {
    type Output = N;

    #[inline]
    fn dot(&self, other: &Self) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

impl<N> Vector<N> for Vector2<N>
where
    N: PrimaFloat,
{
    type NormalizedOutput = Vector2<N>;

    /// Returns the squared magnitude of the vector.
    fn magnitude_squared(&self) -> N {
        self.x * self.x + self.y * self.y
    }

    fn magnitude(&self) -> N {
        self.magnitude_squared().sqrt()
    }

    fn normalize(&self) -> Self::NormalizedOutput {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }
}


impl<N> Into<Point2<N>> for Vector2<N> {
    fn into(self) -> Point2<N> {
        Point2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl<N> PartialEq<Direction> for Vector2<N> where N: PrimaNum {
    fn eq(&self, other: &Direction) -> bool {
        match other {
            Direction::Left => self.x < N::zero() && self.y == N::zero(),
            Direction::Right => self.x > N::zero() && self.y == N::zero(),
            Direction::Up => self.y > N::zero() && self.x == N::zero(),
            Direction::Down => self.y < N::zero() && self.x == N::zero(),
            _ => false
        }
    }
}