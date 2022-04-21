use crate::{Point, PrimaNum, Vector};
use std::marker::PhantomData;

/// A ray, cast in one direction.
pub struct Ray<N>
where
    N: PrimaNum,
{
    /// The starting point of the ray.
    pub origin: Point<N>,
    /// The direction of the ray.
    pub direction: Vector<N>,

    phantom: PhantomData<N>,
}
