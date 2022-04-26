#![warn(missing_docs)]
#![recursion_limit = "16"]

//! Prima is a geometry library aimed at providing a simple, generic and safe API for both 2D and 3D euler-based graphics.
//! Emphasis is on axis-aligned opperations, but the library is designed to be used with any kind of geometry.

/// Base utilities.
mod common;
/// Geometry utilities.
mod geometry;

/// Useful abstractions.
mod abstracts;

/// Implimentations for cross-geometry operations.
mod interactions;

/// Compatability module for external crates, such as glam.
mod compat;

pub use common::*;
pub use geometry::*;
pub use interactions::*;
pub use compat::*;
pub use abstracts::*;

/// Common use types.
pub mod prelude {
    pub use crate::{
        common::PrimaFloat, PrimaNum, Cross, Dot, Shape,
        geometry::Vector, Point, Extent,
        abstracts::Rotation,
    };
}