use crate::{
    core::{AngleMat, Rotation},
    nums::{PrimaFloat, PrimaNum},
    traits::{Magnitude, Cross},
    xy_impl,
};
use std::ops::{AddAssign, Neg, Sub, SubAssign};

/// A vector in 2 dimensions.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vector<N> {
    /// The vector's x component.
    pub x: N,
    /// The vector's y component.
    pub y: N,
}

xy_impl!(Vector);

impl<N> Vector<N>
where
    N: PrimaFloat,
{
    /// Normalize the vector.
    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }
}

impl<N> Magnitude<N> for Vector<N>
where
    N: PrimaFloat,
{
    fn magnitude_squared(&self) -> N {
        self.x * self.x + self.y * self.y
    }
}

impl<N> Add for Vector<N>
where
    N: PrimaNum,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<N> AddAssign for Vector<N>
where
    N: PrimaNum + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<N> Sub for Vector<N>
where
    N: PrimaNum,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<N> SubAssign for Vector<N>
where
    N: PrimaNum + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
