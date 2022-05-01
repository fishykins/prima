#![warn(missing_docs)]
#![recursion_limit = "16"]

//! Prima is a geometry library aimed at providing a simple, generic and safe API for both 2D and 3D euler-based graphics.
//! Emphasis is on axis-aligned opperations, but the library is designed to be used with any kind of geometry.

/// Core geometric components of the library.
pub mod core;

/// Geometry primitives.
pub mod shapes;

/// various utility structs and enums for common geometric concepts.
pub mod abstracts;

/// Useful macros for geometric types (mainly internal use).
pub mod macros;

/// Numerical traits to help keep things as generic as possible.
pub mod nums;

/// A collection of traits that apply to geometric types.
pub mod traits;