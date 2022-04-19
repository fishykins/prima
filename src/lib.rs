#![warn(missing_docs)]
#![recursion_limit = "16"]

//! Prima is a geometry library aimed at providing a simple, generic and safe API for both 2D and 3D euler-based graphics.
//! Emphasis is on axis-aligned opperations, but the library is designed to be used with any kind of geometry.

/// Base utilities.
mod base;
/// Geometry utilities.
mod geometry;

/// Implimentations for cross-geometry operations.
mod interactions;

/// Compatability module for external crates, such as glam.
mod compat;

pub use base::*;
pub use geometry::*;
pub use interactions::*;
pub use compat::*;
