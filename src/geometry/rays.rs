use std::marker::PhantomData;

use num_traits::Num;

use crate::{Point2, Vector2, Point, Vector};


/// Alias for a 2D Ray.
pub type Ray2<N> = Ray<N, Vector2<N>, Point2<N>>;


/// A ray, cast in one direction.
pub struct Ray<N, V, P>
where
    N: Num,
    P: Point<N>,
    V: Vector,
{
    /// The starting point of the ray.
    pub origin: P,
    /// The direction of the ray.
    pub direction: V,

    phantom: PhantomData<N>,
}
