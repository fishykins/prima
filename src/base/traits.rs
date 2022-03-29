use std::ops::{Add, Mul, Sub};
use crate::{Aabr, AxisValue, Point2, PrimaNum};

/// A trait that represents any coordinate based unit of measurement.
pub trait Distance<Rhs = Self> {
    /// The output value.
    type Output;
    /// Computes the manhattan distance between two points.
    fn manhatten_distance(&self, other: &Rhs) -> Self::Output;
    /// Computes the squared euclidean distance between two points.
    fn distance_squared(&self, other: &Rhs) -> Self::Output;
}

/// A trait that represents distance between two points.
pub trait FloatDistance<Rhs = Self>: Distance<Rhs> {
    /// Computes the euclidean distance between two points.
    fn distance(&self, other: &Rhs) -> Self::Output;
}

/// A trait for structs that can have magnitude.
pub trait Vector {
    /// The output value.
    type Output;
    /// Computes the squared magnitude of the vector.
    fn magnitude_squared(&self) -> Self::Output;
    /// Computes the magnitude of the vector.
    fn magnitude(&self) -> Self::Output;
}

/// A trait to represent a point in any space.
pub trait Point<N>:
    Sub<Output = Self> + Add<Output = Self> + Mul<N, Output = Self> + Sized + Copy
where
    N: PrimaNum,
{
    /// Returns the cross product of two points.
    fn cross_product(&self, other: &Self) -> N;
}

/// A trait that implements the dot product of two points.
pub trait Dot<Rhs = Self> {
    /// The output value.
    type Output;
    /// Computes the dot product of two values.
    fn dot(&self, other: &Rhs) -> Self::Output;
}

/// A trait that ensures that the type is a point.
pub trait Coordinate<N>
where
    N: num_traits::Num,
{
    /// The axis this point type covers.
    fn axis(&self) -> AxisValue<N>;
}

/// A trait to denote collisions between two geometric objects.
pub trait Collide<Rhs = Self> {
    /// The output value.
    type Output;
    /// Computes the collision between two geometric objects.
    fn collision(&self, other: &Rhs) -> Option<Self::Output>;

    /// Returns true if the two objects collide.
    fn collides(&self, other: &Rhs) -> bool {
        self.collision(other).is_some()
    }
}

/// A two dimensional shape that can be used for collision detection.
pub trait Shape2<N>
where
    N: PrimaNum,
{
    /// The area of the shape.
    fn area(&self) -> N;
    /// The circumference of the shape.
    fn circumference(&self) -> N;
    /// The center of the shape.
    fn center(&self) -> Point2<N>;
    /// A bounding box that contains the shape.
    fn bounding_box(&self) -> Aabr<N>;
    /// Returns true if the shape contains the point.
    fn contains_point(&self, point: &Point2<N>) -> bool;
}
