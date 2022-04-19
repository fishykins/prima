use crate::{Point, Point2, PrimaNum, Vector, Vector2};
use std::marker::PhantomData;

/// Alias for a 2D Ray.
pub type Ray2<N> = Ray<N, Vector2<N>, Point2<N>>;

/// A ray, cast in one direction.
pub struct Ray<N, V, P>
where
    N: PrimaNum,
    P: Point<N>,
    V: Vector<N>,
{
    /// The starting point of the ray.
    pub origin: P,
    /// The direction of the ray.
    pub direction: V,

    phantom: PhantomData<N>,
}
