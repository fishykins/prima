use std::hash::Hash;

use crate::{
    common::{FastDistance, Distance},
    xy_ops_impl, PrimaFloat, PrimaNum, Dot, Cross,
};
use serde::{Deserialize, Serialize};

use super::Vector;

/// A base struct for 2D points/vectors.
#[derive(Debug, Clone, Copy, Default, PartialEq, Deserialize, Serialize)]
pub struct Point<N = super::DefaultFloat> {
    /// The X coordinate.
    pub x: N,
    /// The Y coordinate.
    pub y: N,
}

xy_ops_impl!(Point);

impl<N> Point<N>
where
    N: PrimaFloat,
{
    /// Returns the vector from the origin to the point.
    pub fn vector(&self, other: Self) -> Vector<N> {
        Vector {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    /// Checks if two points are aligned.
    pub fn aligned(&self, other: &Self) -> bool {
        self.x == other.x || self.y == other.y
    }
}

impl Cross<Point<f32>> for f32 {
    type Product = Point<f32>;

    fn cross_product(&self, other: &Point<f32>) -> Self::Product {
        Point {
            x: -self * other.y,
            y: other.x * *self,
        }
    }
}

impl Cross<Point<f64>> for f64 {
    type Product = Point<f64>;

    fn cross_product(&self, other: &Point<f64>) -> Self::Product {
        Point {
            x: -self * other.y,
            y: other.x * *self,
        }
    }
}

impl<N> FastDistance for Point<N>
where
    N: PrimaNum,
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

impl<N> Distance for Point<N>
where
    N: PrimaFloat,
{
    fn distance(&self, other: &Self) -> Self::Output {
        self.distance_squared(other).sqrt()
    }
}

impl<N> Hash for Point<N>
where
    N: PrimaNum + Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<N> Dot for Point<N>
where
    N: PrimaNum,
{
    type Output = N;

    fn dot(&self, other: &Self) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

impl<N> Eq for Point<N> where N: PrimaNum + Eq {}
// ===========================================================================
// ============================= IMPL VEC ====================================
// ===========================================================================
impl<N> Add<Vector<N>> for Point<N>
where
    N: Add<Output = N>,
{
    type Output = Self;

    fn add(self, other: Vector<N>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<N> Sub<Vector<N>> for Point<N>
where
    N: Sub<Output = N>,
{
    type Output = Self;

    fn sub(self, other: Vector<N>) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<N> Into<Vector<N>> for Point<N> {
    fn into(self) -> Vector<N> {
        Vector {
            x: self.x,
            y: self.y,
        }
    }
}
