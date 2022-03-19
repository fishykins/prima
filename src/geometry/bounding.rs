use num_traits::Num;

use crate::{base::Distance, Collide, Shape2};

use super::Point2;

/// Axis-aligned bounding rectangle.
pub type Aabr<N> = BoundingBox<Point2<N>>;

/// Axis-aligned bounding thingy
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox<P>
where
    P: Distance,
{
    /// The minimum point of the box.
    pub min: P,
    /// The maximum point of the box.
    pub max: P,
}

impl<N> Aabr<N>
where
    N: Num + PartialOrd + Copy,
{
    /// Constructs a new bounding box.
    pub fn new(min: Point2<N>, max: Point2<N>) -> Self {
        Self { min, max }
    }

    /// Validates the bounding box to avoid having a negative width or height.
    pub fn validate(mut self) -> Self {
        if self.min.x > self.max.x {
            std::mem::swap(&mut self.min.x, &mut self.max.x);
        }
        if self.min.y > self.max.y {
            std::mem::swap(&mut self.min.y, &mut self.max.y);
        }
        self
    }

    /// Checks if the bounding box is valid (i.e. has a positive width and height).
    pub fn is_valid(&self) -> bool {
        self.min.x <= self.max.x && self.min.y <= self.max.y
    }

    /// Returns the width of the bounding box.
    pub fn width(&self) -> N {
        self.max.x - self.min.x
    }

    /// Returns the height of the bounding box.
    pub fn height(&self) -> N {
        self.max.y - self.min.y
    }
}

impl<N> Shape2<N> for Aabr<N>
where
    N: Num + Copy + Ord,
{
    fn center(&self) -> Point2<N> {
        let two = N::one() + N::one();
        Point2::new(
            self.min.x + (self.width() / two),
            self.min.y + (self.height() / two),
        )
    }

    fn area(&self) -> N {
        self.width() * self.height()
    }

    fn circumference(&self) -> N {
        let two = N::one() + N::one();
        two * (self.width() + self.height())
    }

    fn bounding_box(&self) -> Aabr<N> {
        self.clone()
    }

    fn contains_point(&self, point: &Point2<N>) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }
}

impl<N> Collide for Aabr<N>
where
    N: num_traits::Num + PartialOrd + Copy,
{
    type Output = Aabr<N>;

    fn collision(&self, other: &Self) -> Option<Self::Output> {
        if self.min.x > other.max.x
            || self.max.x < other.min.x
            || self.min.y > other.max.y
            || self.max.y < other.min.y
        {
            return None;
        }

        // This looks verbose, but it allows us to avoid requiring Ord for N.
        let min_x = if self.min.x > other.min.x {
            self.min.x
        } else {
            other.min.x
        };

        let min_y = if self.min.y > other.min.y {
            self.min.y
        } else {
            other.min.y
        };
        
        let max_x  = if self.max.x < other.max.x {
            self.max.x
        } else {
            other.max.x
        };

        let max_y = if self.max.y < other.max.y {
            self.max.y
        } else {
            other.max.y
        };

        Some(Aabr::new(Point2::new(min_x, min_y), Point2::new(max_x, max_y)))
    }
}
