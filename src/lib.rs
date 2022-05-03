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

/// A collection of commonly used imports for this library.
pub mod prelude {
    pub use crate::core::{Angle, Collision, Line, Point, Rotation, Vector};
    pub use crate::nums::{PrimaFloat, PrimaNum};
    pub use crate::shapes::{Aabr, Circle, Obr};
    pub use crate::traits::{
        Collide, Curved, Distance, Flat, LocalPosition, LocalRotation, Magnitude, Nearest, Shape,
    };
}
