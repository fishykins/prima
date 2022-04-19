mod bounding;
mod lines;
mod point_2;
mod rays;
mod rect;
mod triangle;
mod vector_2;
mod circle;
mod collision;

#[cfg(feature = "d3")]
mod point_3;
#[cfg(feature = "d3")]
mod vector_3;
#[cfg(feature = "d3")]
mod cube;

pub use circle::*;
pub use bounding::*;
pub use lines::*;
pub use point_2::Point2;
pub use rays::*;
pub use rect::*;
pub use triangle::*;
pub use vector_2::*;
pub use collision::*;

#[cfg(feature = "d3")]
pub use point_3::Point3;
#[cfg(feature = "d3")]
pub use cube::Cube;
#[cfg(feature = "d3")]
pub use vector_3::Vector3;

/// The default float to use when none is specified.
pub type DefaultFloat = f32;