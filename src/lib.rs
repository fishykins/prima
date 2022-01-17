#![warn(missing_docs)]
#![recursion_limit = "16"]

//! Prima is yet another geometry library, built around the core principle of simple code and readability. It provides helper structs
//! for primitive shapes, basic collision detection and a suite of graph structures. The graphs are vector orientated and are built with
//! procedural world building in mind. Generic floats have been avoided in favour of explicitly typed structs, following the example of 
//! [glam](https://crates.io/crates/glam). Support for the crate [vek](https://crates.io/crates/vek) may be implimented in future updates.
/// Core data used accross the board. 
pub mod core;
/// A set of graphs used to define tree-maps, vector graphs and voronoi diagrams.
pub mod graphs;
/// Simple helper structs and functions to handle geometric shapes.
pub mod geom;