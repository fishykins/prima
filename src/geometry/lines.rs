use crate::base::{Distance, Vector};

use super::{Point2, Vector2};

/// Alias for a 2D point.
pub type Line2<N> = Line::<Point2<N>>;

/// Alias for a 2D Ray.
pub type Ray2<N> = Ray::<Point2<N>, Vector2<N>>;

/// A line from point to point.
#[derive(Clone, Debug, PartialEq)]
pub struct Line<P> where P: Distance {
    /// The starting point of the line.
    pub start: P,
    /// The ending point of the line.
    pub end: P,
}

/// A ray, cast in one direction.
pub struct Ray<P, V> where P: Distance, V: Vector {
    /// The starting point of the ray.
    pub origin: P,
    /// The direction of the ray.
    pub direction: V,
}