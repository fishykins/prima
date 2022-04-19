use serde::{Deserialize, Serialize};

use crate::{xyz_ops_impl, common::{Dot, Vector}, PrimaFloat, PrimaNum, Point3, FastDistance, Distance, abstracts::Direction};
use super::Point2;

/// A base struct for 2D points/vectors.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, Serialize)]
pub struct Vector3<N = super::DefaultFloat> {
    /// The X magnitude.
    pub x: N,
    /// The Y magnitude.
    pub y: N,
    /// The Z magnitude.
    pub z: N,
}

xyz_ops_impl!(Vector3);

impl<N> Dot for Vector3<N> where N: PrimaNum {
    type Output = N;

    #[inline]
    fn dot(&self, other: &Self) -> Self::Output {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<N> Vector for Vector3<N>
where
    N: PrimaFloat,
{
    type Output = N;

    /// Returns the squared magnitude of the vector.
    fn magnitude_squared(&self) -> Self::Output {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn magnitude(&self) -> Self::Output {
        self.magnitude_squared().sqrt()
    }
}


impl<N> Into<Point2<N>> for Vector3<N> {
    fn into(self) -> Point2<N> {
        Point2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl<N> Into<Point3<N>> for Vector3<N> {
    fn into(self) -> Point3<N> {
        Point3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl<N> PartialEq<Direction> for Vector3<N> where N: PrimaNum {
    fn eq(&self, other: &Direction) -> bool {
        match other {
            Direction::Left => self.x < N::zero() && self.y == N::zero() && self.z == N::zero(),
            Direction::Right => self.x > N::zero() && self.y == N::zero() && self.z == N::zero(),
            Direction::Up => self.y > N::zero() && self.x == N::zero() && self.z == N::zero(),
            Direction::Down => self.y < N::zero() && self.x == N::zero() && self.z == N::zero(),
            Direction::Forward => self.z > N::zero() && self.x == N::zero() && self.y == N::zero() && self.z > N::zero(),
            Direction::Backward => self.z < N::zero() && self.x == N::zero() && self.y == N::zero() && self.z < N::zero(),
        }
    }
}


impl<N> Point3<N>
where
    N: PrimaFloat,
{
    /// Returns the vector from the origin to the point.
    pub fn vector(&self, other: Self) -> Vector3<N> {
        Vector3 {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }
}

impl<N> FastDistance for Point3<N>
where
    N: PrimaNum,
{
    type Output = N;

    fn distance_squared(&self, other: &Self) -> Self::Output {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
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
        let dz = if self.z >= other.z {
            self.z - other.z
        } else {
            other.z - self.z
        };
        dx + dy + dz
    }
}

impl<N> Distance for Point3<N>
where
    N: PrimaFloat,
{
    fn distance(&self, other: &Self) -> Self::Output {
        self.distance_squared(other).sqrt()
    }
}
// ===========================================================================
// ============================= IMPL VEC ====================================
// ===========================================================================
impl<N> Add<Vector3<N>> for Point3<N>
where
    N: Add<Output = N>,
{
    type Output = Self;

    fn add(self, other: Vector3<N>) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<N> Sub<Vector3<N>> for Point3<N>
where
    N: Sub<Output = N>,
{
    type Output = Self;

    fn sub(self, other: Vector3<N>) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<N> Into<Vector3<N>> for Point3<N> {
    fn into(self) -> Vector3<N> {
        Vector3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}
