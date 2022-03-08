use crate::AxisValue;

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
pub trait Vector<Rhs = Self> {
    /// The output value.
    type Output;
    /// Computes the squared magnitude of the vector.
    fn magnitude_squared(&self) -> Self::Output;
    /// Computes the magnitude of the vector.
    fn magnitude(&self) -> Self::Output;
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
