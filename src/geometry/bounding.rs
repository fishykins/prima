use core::panic;

use super::Point;
use crate::{
    common::FastDistance, Collision, Interact, Intersect, PrimaFloat, PrimaNum, Shape2, Vector, Line,
};
use serde::{Deserialize, Serialize};

/// Axis-aligned bounding rectangle.
pub type Aabr<N = super::DefaultFloat> = BoundingBox<Point<N>>;

/// Axis-aligned bounding thingy
#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct BoundingBox<P>
where
    P: FastDistance,
{
    /// The minimum point of the box.
    pub min: P,
    /// The maximum point of the box.
    pub max: P,
}

impl<N> Aabr<N>
where
    N: PrimaNum,
{
    /// Constructs a new bounding box.
    pub fn new(min: Point<N>, max: Point<N>) -> Self {
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

    /// Returns the half-extents of the bounding box.
    pub fn half_extents(&self) -> (N, N) {
        let two = N::one() + N::one();
        (self.width() / two, self.height() / two)
    }

    /// Returns any overlap between the two bounding boxes.
    pub fn common_bounds(&self, other: &Self) -> Option<Self> {
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

        let max_x = if self.max.x < other.max.x {
            self.max.x
        } else {
            other.max.x
        };

        let max_y = if self.max.y < other.max.y {
            self.max.y
        } else {
            other.max.y
        };

        Some(Self::new(
            Point::new(min_x, min_y),
            Point::new(max_x, max_y),
        ))
    }
}

impl<N> Shape2<N> for Aabr<N>
where
    N: PrimaNum,
{
    fn center(&self) -> Point<N> {
        let two = N::one() + N::one();
        Point::new(
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

    fn contains_point(&self, point: &Point<N>) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
    }
}

impl<N> Interact<N> for Aabr<N>
where
    N: PrimaFloat,
{
    fn collision(&self, other: &Self) -> Option<Collision<N>> {
        let n = other.center() - self.center();
        // Calculate half extents along x axis for each shape.
        let a_extent = self.half_extents();
        let b_extent = other.half_extents();

        // Calculate overlap on x axis.
        let x_overlap = a_extent.0 + b_extent.0 - n.x.abs();

        if x_overlap > N::zero() {
            // Calculate overlap on y axis.
            let y_overlap = a_extent.1 + b_extent.1 - n.y.abs();

            if y_overlap > N::zero() {
                // Find out which axis is axis of least penetration
                if x_overlap < y_overlap {
                    let normal = if n.x < N::zero() {
                        Vector::new(-N::one(), N::zero())
                    } else {
                        Vector::new(N::one(), N::zero())
                    };
                    return Some(Collision {
                        penetration: x_overlap,
                        normal,
                    });
                } else {
                    let normal = if n.y < N::zero() {
                        Vector::new(N::zero(), -N::one())
                    } else {
                        Vector::new(N::zero(), N::one())
                    };
                    return Some(Collision {
                        penetration: y_overlap,
                        normal,
                    });
                }
            }
        }
        None
    }

    // TODO: This needs testing as it is a fish original.
    fn nearest_extent(&self, other: &Self) -> Point<N> {
        let x_dist = (other.center().x - self.center().x).abs();
        let y_dist = (other.center().y - self.center().y).abs();

        if x_dist == N::zero() && y_dist == N::zero() {
            return self.center();
        }

        if x_dist == y_dist {
            // We are on a corner, do something about it!
            return Point::new(
                self.center().x + (self.half_extents().0 * (other.center().x - self.center().x).signum()),
                self.center().y + (self.half_extents().1 * (other.center().y - self.center().y).signum()),
            );
        }

        // TODO: Maybe try and condense this using signum, as done above?
        let edge = if x_dist > y_dist {
            if other.center().x > self.center().x {
                // Left
                Line::new(
                    self.min,
                    self.min + Vector::new(N::zero(), self.height()),
                )
            } else {
                // Right
                Line::new(
                    self.min + Vector::new(self.width(), N::zero()),
                    self.min + Vector::new(self.width(), self.height()),
                )
            }
        } else {
            if other.center().y > self.center().y {
                // Bottom
                Line::new(
                    self.min,
                    self.min + Vector::new(self.width(), N::zero()),
                )
            } else {
                // Top
                Line::new(
                    self.min + Vector::new(N::zero(), self.height()),
                    self.min + Vector::new(self.width(), self.height()),
                )
            }
        };

        let ray = Line::new(other.center(), self.center());
        if let Some(intersection) = edge.contact_point(&ray) {
            return intersection;
        }
        panic!("No contact point found- this should not happen!");
    }
}

impl<N> Intersect for Aabr<N>
where
    N: PrimaNum,
{
    fn intersecting(&self, other: &Self) -> bool {
        if self.max.x < other.min.x || self.min.x > other.max.x {
            return false;
        }
        if self.max.y < other.min.y || self.min.y > other.max.y {
            return false;
        }
        true
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aabr_test() {
        let a = Aabr::new(Point::new(0.0, 0.0), Point::new(1.0, 1.0));
        let b = Aabr::new(Point::new(0.5, 0.5), Point::new(1.5, 1.5));
        assert!(a.intersecting(&b));
    }
}