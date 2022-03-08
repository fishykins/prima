use crate::base::Distance;

use super::Point2;

/// Axis-aligned bounding rectangle.
pub type Aabr<N> = BoundingBox<Point2<N>>;

/// Axis-aligned bounding thingy
pub struct BoundingBox<P> where P: Distance {
    /// The minimum point of the box.
    pub min: P,
    /// The maximum point of the box.
    pub max: P,
}

impl<N> Aabr<N> where N: num_traits::Num + PartialOrd + Copy {
    /// Constructs a new bounding box.
    pub fn new(min: Point2<N>, max: Point2<N>) -> Self {
        Self {
            min,
            max,
        }
    }
}