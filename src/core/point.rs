use crate::{
    core::{AngleMat, Line, Rotation, Vector},
    nums::{PrimaFloat, PrimaNum},
    traits::{Distance, Magnitude, Shape},
    xy_impl,
};
use std::ops::{AddAssign, Neg, Sub, SubAssign};

use super::Extent;

/// A point in 2D space.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point<N> {
    /// The x coordinate of the point.
    pub x: N,
    /// The y coordinate of the point.
    pub y: N,
}

xy_impl!(Point);

impl<N> Point<N>
where
    N: PrimaFloat,
{
    /// Returns the relative point to other, which is shorthand
    /// for translating self by the vector from self to other.
    pub fn relative_to(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    /// Checks if the point lies on the given line.
    pub fn on_line(&self, line: &Line<N>) -> bool {
        let line_length = line.magnitude();
        let dist_start = line.start.distance(self);
        let dist_end = line.end.distance(self);
        let buffer = N::from_f32(0.01).unwrap();
        dist_start + dist_end >= line_length - buffer
            && dist_start + dist_end <= line_length + buffer
    }

    /// Checks if the point is inside the given shape.
    pub fn in_shape(&self, shape: impl Shape<N>) -> bool {
        shape.contains(self)
    }
}

impl<N> Distance<N, Self> for Point<N>
where
    N: PrimaFloat,
{
    fn squared_distance(&self, other: &Self) -> N {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

//==============================================================================//
//=============================== OPPERATIONS ==================================//
//==============================================================================//

impl<N> Add<Vector<N>> for Point<N>
where
    N: PrimaNum,
{
    type Output = Self;

    fn add(self, rhs: Vector<N>) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<N> AddAssign<Vector<N>> for Point<N>
where
    N: PrimaNum + AddAssign,
{
    fn add_assign(&mut self, rhs: Vector<N>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<N> Add<Extent<N>> for Point<N>
where
    N: PrimaNum,
{
    type Output = Self;

    fn add(self, rhs: Extent<N>) -> Self {
        Self {
            x: self.x + rhs.half_width(),
            y: self.y + rhs.half_height(),
        }
    }
}

impl<N> AddAssign<Extent<N>> for Point<N>
where
    N: PrimaNum + AddAssign,
{
    fn add_assign(&mut self, rhs: Extent<N>) {
        self.x += rhs.half_width();
        self.y += rhs.half_height();
    }
}

impl<N> Sub for Point<N>
where
    N: PrimaNum,
{
    type Output = Vector<N>;

    fn sub(self, rhs: Self) -> Vector<N> {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<N> SubAssign<Vector<N>> for Point<N>
where
    N: PrimaNum + SubAssign,
{
    fn sub_assign(&mut self, rhs: Vector<N>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<N> Sub<Extent<N>> for Point<N>
where
    N: PrimaNum,
{
    type Output = Self;

    fn sub(self, rhs: Extent<N>) -> Self {
        Self {
            x: self.x - rhs.half_width(),
            y: self.y - rhs.half_height(),
        }
    }
}

impl<N> SubAssign<Extent<N>> for Point<N>
where
    N: PrimaNum + SubAssign,
{
    fn sub_assign(&mut self, rhs: Extent<N>) {
        self.x -= rhs.half_width();
        self.y -= rhs.half_height();
    }
}
