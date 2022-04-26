mod aabr;
mod circle;
mod collision;
mod extent;
mod line;
mod matrix;
mod obr;
mod point;
mod rays;
mod rect;
mod rotation;
mod triangle;
mod vector;

pub use aabr::*;
pub use circle::*;
pub use collision::*;
pub use extent::*;
pub use line::*;
pub use matrix::*;
pub use obr::*;
pub use point::*;
pub use rays::*;
pub use rect::*;
pub use rotation::*;
pub use triangle::*;
pub use vector::*;

/// The default float to use when none is specified.
pub type DefaultFloat = f32;
