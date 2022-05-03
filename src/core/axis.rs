use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, DivAssign, Div};

use crate::{
    nums::{PrimaFloat, PrimaNum},
    traits::Flat,
};

use super::{Vector, Line};

/// A single axis line between two values. Useful for collision detection, especially when using seperating axis theorem.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AxisLine<N> {
    /// The start value of the axis.
    pub start: N,
    /// The end value of the axis.
    pub end: N,
}

impl<N> AxisLine<N>
where
    N: PrimaNum,
{
    /// Creates a new axis segment.
    pub fn new(start: N, end: N) -> Self {
        Self { start, end }
    }

    /// Returns true if the two segments are intersecting.
    pub fn intersecting(&self, other: &Self) -> bool {
        self.start <= other.start && other.start <= self.end
            || self.start <= other.end && other.end <= self.end
    }

    /// Returns true if the value lies on the axis.
    pub fn contains_point(&self, value: N) -> bool {
        self.start <= value && value <= self.end
    }

    /// Returns true if this axis totally envelops the other.
    pub fn contains_line(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    /// Returns the length/magnitude of the line.
    pub fn magnitude(&self) -> N {
        self.end - self.start
    }
}

impl<N> AxisLine<N> where N: PrimaFloat {
    /// Converts this axis line to a line along the given vector.
    pub fn to_line(&self, vector: Vector<N>) -> Line<N> {
        let axis = vector.normalize();
        let start = axis * self.start;
        let end = axis * self.end;
        Line::new(start.as_point(), end.as_point())
    }
}

impl<N> Add<N> for AxisLine<N> where N: PrimaNum {
    type Output = Self;

    fn add(self, rhs: N) -> Self {
        Self {
            start: self.start + rhs,
            end: self.end + rhs,
        }
    }
}

impl<N> AddAssign<N> for AxisLine<N> where N: PrimaNum {
    fn add_assign(&mut self, rhs: N) {
        self.start += rhs;
        self.end += rhs;
    }
}

impl<N> Sub<N> for AxisLine<N> where N: PrimaNum {
    type Output = Self;

    fn sub(self, rhs: N) -> Self {
        Self {
            start: self.start - rhs,
            end: self.end - rhs,
        }
    }
}

impl<N> SubAssign<N> for AxisLine<N> where N: PrimaNum {
    fn sub_assign(&mut self, rhs: N) {
        self.start -= rhs;
        self.end -= rhs;
    }
}

impl<N> Mul<N> for AxisLine<N> where N: PrimaNum {
    type Output = Self;

    fn mul(self, rhs: N) -> Self {
        Self {
            start: self.start * rhs,
            end: self.end * rhs,
        }
    }
}

impl<N> MulAssign<N> for AxisLine<N> where N: PrimaNum {
    fn mul_assign(&mut self, rhs: N) {
        self.start = self.start * rhs;
        self.end = self.end * rhs;
    }
}

impl<N> Div<N> for AxisLine<N> where N: PrimaNum {
    type Output = Self;

    fn div(self, rhs: N) -> Self {
        Self {
            start: self.start / rhs,
            end: self.end / rhs,
        }
    }
}

impl<N> DivAssign<N> for AxisLine<N> where N: PrimaNum {
    fn div_assign(&mut self, rhs: N) {
        self.start = self.start / rhs;
        self.end = self.end / rhs;
    }
}

/// Projects a flat shape onto an axis, returning the minimum and maximum values.
pub fn project_verts_to_axis<N>(shape: impl Flat<N>, axis: Vector<N>) -> AxisLine<N>
where
    N: PrimaFloat,
{
    let a = axis.normalize();
    let mut min = N::infinity();
    let mut max = N::neg_infinity();

    // Get all of self's points along each axis
    for vert in shape.vertices().iter() {
        let v: Vector<N> = Vector::new(vert.x, vert.y);
        let x = v.dot(&a);

        if x < min {
            min = x;
        } else if x > max {
            max = x;
        }
    }
    AxisLine::new(min, max)
}

/// Projects a flat shape onto a pair of axis, returning the minimum and maximum values.
pub fn project_shape_to_axis_pair<N>(
    shape: &impl Flat<N>,
    axis_x: Vector<N>,
    axis_y: Vector<N>,
) -> (AxisLine<N>, AxisLine<N>)
where
    N: PrimaFloat,
{
    let a_x = axis_x.normalize();
    let a_y = axis_y.normalize();

    let mut min_x = N::infinity();
    let mut max_x = N::neg_infinity();
    let mut min_y = N::infinity();
    let mut max_y = N::neg_infinity();

    // Get all of self's points along each axis
    for vert in shape.vertices().iter() {
        let v: Vector<N> = Vector::new(vert.x, vert.y);
        let x = v.dot(&a_x);
        let y = v.dot(&a_y);

        if x < min_x {
            min_x = x;
        } else if x > max_x {
            max_x = x;
        }

        if y < min_y {
            min_y = y;
        } else if y > max_y {
            max_y = y;
        }
    }
    (AxisLine::new(min_x, max_x), AxisLine::new(min_y, max_y))
}
