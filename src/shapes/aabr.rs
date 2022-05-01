use crate::{
    core::{Collision, Extent, Point, Vector},
    nums::{PrimaFloat, PrimaNum},
    traits::{Collide, Distance, Flat, LocalPosition, Nearest, Shape},
};

use super::Circle;

/// An axis-aligned bounding rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Aabr<N> {
    /// The minimum point of the rectangle.
    pub min: Point<N>,
    /// The maximum point of the rectangle.
    pub max: Point<N>,
}

impl<N> Aabr<N>
where
    N: PrimaNum
{
    /// Creates a new Aabr from a center point, width and height.
    pub fn from_point(center: Point<N>, width: N, height: N) -> Self {
        let e = Extent::<N>::new(width, height).half();
        Self {
            min: center - e,
            max: center + e,
        }
    }

    /// Creates a new Aabr from a min and max point.
    pub fn new(min: Point<N>, max: Point<N>) -> Self {
        Self {
            min,
            max
        }
    }

    /// Returns the min max pair.
    pub fn min_max(&self) -> (Point<N>, Point<N>) {
        (self.min, self.max)
    }

    /// Returns the width of the Aabr.
    pub fn width(&self) -> N {
        self.max.x - self.min.x
    }

    /// Returns the height of the Aabr.
    pub fn height(&self) -> N {
        self.max.y - self.min.y
    }

    /// Returns the extent of this Aabr.
    pub fn extent(&self) -> Extent<N> {
        Extent::new(self.width(), self.height())
    }

    /// Returns any overlap between the two bounding boxes.
    pub fn overlap(&self, other: &Self) -> Option<Self> {
        let (min_a, max_a) = self.min_max();
        let (min_b, max_b) = other.min_max();
        if min_a.x > max_b.x
            || max_a.x < min_b.x
            || min_a.y > max_b.y
            || max_a.y < min_b.y
        {
            return None;
        }
        Some(self.overlap_unchecked(other))
    }

    /// Returns the overlap of two bounding boxes, without checking for validity.
    pub fn overlap_unchecked(&self, other: &Self) -> Self {
        let (min_a, max_a) = self.min_max();
        let (min_b, max_b) = other.min_max();

        // This looks verbose, but it allows us to avoid requiring Ord for N.
        let min_x = if min_a.x > min_b.x {
            min_a.x
        } else {
            min_b.x
        };

        let min_y = if min_a.y > min_b.y {
            min_a.y
        } else {
            min_b.y
        };

        let max_x = if max_a.x < max_b.x {
            max_a.x
        } else {
            max_b.x
        };

        let max_y = if max_a.y < max_b.y {
            max_a.y
        } else {
            max_b.y
        };

        Self::new(Point::new(min_x, min_y), Point::new(max_x, max_y))
    }
}

impl<N> Shape<N> for Aabr<N>
where
    N: PrimaFloat,
{
    fn volume(&self) -> N {
        self.extent().volume()
    }

    fn circumference(&self) -> N {
        self.extent().double().sum()
    }

    fn bounding_rect(&self) -> Aabr<N> {
        self.clone()
    }

    fn bounding_circle(&self) -> Circle<N> {
        let center = self.position();
        let radius = center.distance(&self.max);
        Circle::new(center, radius)
    }

    fn contains(&self, point: &Point<N>) -> bool {
        let (min, max) = self.min_max();
        point.x >= min.x && point.x <= max.x && point.y >= min.y && point.y <= max.y
    }
}

impl<N> Flat<N> for Aabr<N>
where
    N: PrimaFloat,
{
    fn vertices(&self) -> Vec<Point<N>> {
        let min = self.min;
        let max = self.max;
        [
            Point::new(min.x, min.y),
            Point::new(min.x, max.y),
            Point::new(max.x, max.y),
            Point::new(max.x, min.y),
        ]
        .into()
    }
}

impl<N> LocalPosition<N> for Aabr<N>
where
    N: PrimaFloat,
{
    fn position(&self) -> Point<N> {
        Point::new(self.min.x + self.max.x, self.min.y + self.max.y) / (N::one() + N::one())
    }
}

impl<N> Distance<N, Point<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn squared_distance(&self, other: &Point<N>) -> N {
        self.nearest_point(other).squared_distance(other)
    }
}

impl<N> Nearest<N, Point<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, point: &Point<N>) -> Point<N> {
        let mut nearest = self.position();
        let min = self.min;
        let max = self.max;
        if point.x < min.x {
            nearest.x = min.x;
        } else if point.x > max.x {
            nearest.x = max.x;
        }
        if point.y < min.y {
            nearest.y = min.y;
        } else if point.y > max.y {
            nearest.y = max.y;
        }
        nearest
    }
}

impl<N> Nearest<N, Aabr<N>> for Aabr<N> where N: PrimaFloat {
    fn nearest_point(&self, _other: &Aabr<N>) -> Point<N> {
        todo!()
    }
}

impl<N> Collide<N> for Aabr<N>
where
    N: PrimaFloat,
{
    fn collision(&self, other: &Self) -> Option<Collision<N>> {
        let n = other.position() - self.position();
        let self_extent = self.extent().half();
        let other_extent = other.extent().half();
        let a_w = self_extent.width();
        let b_w = other_extent.width();
        let a_h = self_extent.height();
        let b_h = other_extent.height();
        
        // Calculate overlap on x axis.
        let x_overlap = a_w + b_w - n.x.abs();

        if x_overlap > N::zero() {
            // Calculate overlap on y axis.
            let y_overlap = a_h + b_h - n.y.abs();

            if y_overlap > N::zero() {
                // We have an overlap on both axes- calculate the area.
                let overlap = self.overlap_unchecked(other);

                // Find out which axis is axis of least penetration
                if x_overlap < y_overlap {
                    let normal = if n.x < N::zero() {
                        Vector::new(-N::one(), N::zero())
                    } else {
                        Vector::new(N::one(), N::zero())
                    };
                    return Some(Collision::new(overlap.position(), normal, x_overlap));
                } else {
                    let normal = if n.y < N::zero() {
                        Vector::new(N::zero(), -N::one())
                    } else {
                        Vector::new(N::zero(), N::one())
                    };
                    return Some(Collision::new(overlap.position(), normal, y_overlap));
                }
            }
        }
        None
    }

    fn enveloping(&self, other: &Self) -> bool {
        let (min_a, max_a) = self.min_max();
        let (min_b, max_b) = other.min_max();
        min_a.x <= min_b.x && max_a.x >= max_b.x && min_a.y <= min_b.y && max_a.y >= max_b.y
    }

    fn enveloped_by(&self, other: &Self) -> bool {
        other.enveloping(self)
    }
}
