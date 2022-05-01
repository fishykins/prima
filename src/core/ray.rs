use crate::nums::PrimaNum;

use super::{Point, Vector};

/// A 2D ray with an origin and a direction.
pub struct Ray<N> {
    /// The origin of the ray.
    pub origin: Point<N>,
    /// The direction of the ray.
    pub direction: Vector<N>,
}

impl<N> Ray<N> where N : PrimaNum {
    /// Creates a new ray with the given origin and direction.
    pub fn new(origin: Point<N>, direction: Vector<N>) -> Self {
        Self {
            origin,
            direction,
        }
    }
}