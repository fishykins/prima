#![warn(missing_docs)]
#![recursion_limit = "16"]

//! Prima is a geometry library aimed at providing a simple, generic and safe API for both 2D and 3D euler-based graphics.

/// Base utilities.
pub mod base;
/// Geometry utilities.
pub mod geometry;

/// Graph structures
#[cfg(feature = "graphs")]
pub mod graphs;