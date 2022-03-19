use num_traits::{Float, Num};

use crate::{
    base::{Distance, FloatDistance},
    xy_ops_impl,
};

use super::Vector2;

/// A base struct for 2D points/vectors.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Point2<N> {
    /// The X coordinate.
    pub x: N,
    /// The Y coordinate.
    pub y: N,
}

xy_ops_impl!(Point2);

impl<N> Point2<N>
where
    N: Float,
{
    /// Returns the vector from the origin to the point.
    pub fn vector(&self, other: Self) -> Vector2<N> {
        Vector2 {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
}

impl<N> Distance for Point2<N>
where
    N: Num + PartialOrd + Copy,
{
    type Output = N;

    fn distance_squared(&self, other: &Self) -> Self::Output {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    fn manhatten_distance(&self, other: &Self) -> Self::Output {
        let dx = if self.x >= other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let dy = if self.y >= other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        dx + dy
    }
}

impl<N> FloatDistance for Point2<N>
where
    N: Float,
{
    fn distance(&self, other: &Self) -> Self::Output {
        self.distance_squared(other).sqrt()
    }
}

// ===========================================================================
// ============================= IMPL VEC ====================================
// ===========================================================================
impl<N> Add<Vector2<N>> for Point2<N>
where
    N: Add<Output = N>,
{
    type Output = Self;

    fn add(self, other: Vector2<N>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<N> Sub<Vector2<N>> for Point2<N>
where
    N: Sub<Output = N>,
{
    type Output = Self;

    fn sub(self, other: Vector2<N>) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<N> Into<Vector2<N>> for Point2<N> {
    fn into(self) -> Vector2<N> {
        Vector2 {
            x: self.x,
            y: self.y,
        }
    }
}