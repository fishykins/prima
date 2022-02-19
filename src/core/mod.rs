mod axis;
mod index_type;
mod ord_num;
mod plane;
mod transverse;

/// A few useful mathematical functions.
pub mod maths;

pub use axis::Axis;
pub use index_type::{DefaultIx, IndexType};
pub use ord_num::OrdNum;
pub use plane::{Plane2D, Plane3D};
pub use transverse::Transverse;
