use serde::{Deserialize, Serialize};
use super::Point;
use crate::{abstracts::Direction, common::Dot, xy_ops_impl, Cross, PrimaFloat, PrimaNum};

/// A base struct for 2D points/vectors.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Vector<N = super::DefaultFloat> {
    /// The X magnitude.
    pub x: N,
    /// The Y magnitude.
    pub y: N,
}

xy_ops_impl!(Vector);

impl<N> Dot for Vector<N>
where
    N: PrimaNum,
{
    type Output = N;

    #[inline]
    fn dot(&self, other: &Self) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

impl<N> Vector<N> where N: PrimaNum {
    /// Returns the counter-clockwise perpendicular vector.
    pub fn perpendicular_cc(self) -> Self {
        Vector {
            x: N::zero() - self.y,
            y: self.x,
        }
    }

    /// Returns the clockwise perpendicular vector.
    pub fn perpendicular(self) -> Self {
        Vector {
            x: self.y,
            y: N::zero() - self.x,
        }
    }

    /// Returns the inverted vector.
    pub fn inverted(self) -> Self {
        Vector {
            x: N::zero() - self.x,
            y: N::zero() - self.y,
        }
    }
}

impl<N> Vector<N>
where
    N: PrimaFloat,
{
    /// Returns the squared magnitude of the vector.
    pub fn magnitude_squared(&self) -> N {
        self.x * self.x + self.y * self.y
    }

    /// The magnitude of the vector.
    pub fn magnitude(&self) -> N {
        self.magnitude_squared().sqrt()
    }

    /// Returns the unit vector of the vector.
    pub fn normalize(self) -> Self {
        if self.magnitude_squared() == N::zero() {
            return Self::zero();
        }
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }
}

impl<N> Into<Point<N>> for Vector<N> {
    fn into(self) -> Point<N> {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

impl<N> PartialEq<Direction> for Vector<N>
where
    N: PrimaNum,
{
    fn eq(&self, other: &Direction) -> bool {
        match other {
            Direction::Left => self.x < N::zero() && self.y == N::zero(),
            Direction::Right => self.x > N::zero() && self.y == N::zero(),
            Direction::Up => self.y > N::zero() && self.x == N::zero(),
            Direction::Down => self.y < N::zero() && self.x == N::zero(),
        }
    }
}
