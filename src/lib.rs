/// Core data used accross the board
pub mod core;

/// 2D geometry library
pub mod geom;

/// Rendering utilities
#[cfg(feature = "rendering")]
pub mod render;

/// Mesh library
pub mod mesh;

/// Graphs
pub mod graphs;

pub extern crate vek;