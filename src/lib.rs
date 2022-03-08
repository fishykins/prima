#![warn(missing_docs)]
#![recursion_limit = "16"]

//! Prima is a geometry library aimed at providing a simple, generic and safe API for both 2D and 3D euler-based graphics.

/// Base utilities.
mod base;
/// Geometry utilities.
mod geometry;

pub use base::*;
pub use geometry::*;
