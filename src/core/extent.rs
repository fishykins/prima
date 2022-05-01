use super::{AngleMat, Rotation};
use crate::{xy_impl, nums::{PrimaNum, PrimaFloat}};
use std::{fmt::Display, ops::Neg};

/// Width and height extent of a shape.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Extent<N> {
    /// Width
    x: N,
    /// Height
    y: N,
}

impl<N> Extent<N>
where
    N: PrimaNum,
{
    #[inline]
    fn two() -> N {
        N::one() + N::one()
    }

    /// The full width of the extent.
    pub fn width(&self) -> N {
        self.x
    }

    /// The full height of the extent.
    pub fn height(&self) -> N {
        self.y
    }

    /// Returns the half-width of the extent.
    pub fn half_width(&self) -> N {
        self.x / Self::two()
    }

    /// Halves the extent.
    pub fn half(&self) -> Self {
        Self {
            x: self.x / Self::two(),
            y: self.y / Self::two(),
        }
    }

    /// Doubles the extent.
    pub fn double(&self) -> Self {
        Self {
            x: self.x * Self::two(),
            y: self.y * Self::two(),
        }
    }

    /// Returns the half-height of the extent.
    pub fn half_height(&self) -> N {
        self.y / Self::two()
    }

    /// The volume of the extent.
    pub fn volume(&self) -> N {
        self.x * self.y
    }

    /// The sum of width and height.
    pub fn sum(&self) -> N {
        self.x + self.y
    }
}

impl<N> Display for Extent<N>
where
    N: PrimaNum,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Extent(width: {}, height: {})",
            self.width(),
            &self.height()
        )
    }
}

xy_impl!(Extent);
