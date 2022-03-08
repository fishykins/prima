/// A trait that represents non-float dependant distance between two points.
pub trait BaseDistance<Rhs = Self> {
    /// The output value.
    type Output;
    /// Computes the manhattan distance between two points.
    fn manhatten_distance(&self, other: &Rhs) -> Self::Output;
    /// Computes the squared euclidean distance between two points.
    fn distance_squared(&self, other: &Rhs) -> Self::Output;
}

/// A trait that represents distance between two points.
pub trait Distance<Rhs = Self>: BaseDistance<Rhs> {
    /// Computes the euclidean distance between two points.
    fn distance(&self, other: &Rhs) -> Self::Output;
}

/// A trait for structs that can have magnitude.
pub trait Magnitude<Rhs = Self> {
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

/// A trait to help distinguish structs that can be used as a vector or coordinate.
pub trait Position {}