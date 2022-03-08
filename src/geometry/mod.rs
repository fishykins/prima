mod point_2;
mod vector_2;

#[cfg(feature = "d3")]
mod point_3;
#[cfg(feature = "d3")]
mod vector_3;

pub use vector_2::*;
pub use point_2::Point2;

#[cfg(feature = "d3")]
pub use point_3::Point3;

/// A Point represents a point in space, using either floating point or integer values for each axis.
#[cfg(not(feature = "d3"))]
pub type Point<N> = Point2<N>;
/// A Point represents a point in space, using either floating point or integer values for each axis.
#[cfg(feature = "d3")]
pub type Point<N> = Point3<N>;

/// A Vector represents a vector in space, using either floating point or integer values for each axis.
#[cfg(not(feature = "d3"))]
pub type Vector<N> = Vector2<N>;

// #[cfg(feature = "d3")]
// pub type Vector<N> = Vector3<N>;
