mod aabr;
mod circle;
mod collision;
mod line;
mod point;
mod rays;
mod rect;
mod triangle;
mod vector;

pub use aabr::*;
pub use circle::*;
pub use collision::*;
pub use line::*;
pub use point::*;
pub use rays::*;
pub use rect::*;
pub use triangle::*;
pub use vector::*;

/// The default float to use when none is specified.
pub type DefaultFloat = f32;
