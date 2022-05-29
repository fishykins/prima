use core::ops::Range;
use num_traits::{Float, FromPrimitive, Num, ToPrimitive, Unsigned};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{AddAssign, Neg, SubAssign};
use std::{f32, f64};

use crate::core::{Point, Vector};
use crate::traits::Cross;

/// A more general trait that covers most number types.
pub trait PrimaNum:
    Num
    + PartialOrd
    + Clone
    + Copy
    + Display
    + Debug
    + FromPrimitive
    + ToPrimitive
    + AddAssign
    + SubAssign
{
    /// The bitsize of this number type.
    const BITS: usize;
}

/// Intiger types.
pub trait PrimaInt: PrimaNum + Hash {}

/// Float types.
pub trait PrimaFloat: PrimaNum + Float + Neg {
    /// A quick and dirty grabber for the value of pi!
    fn pi() -> Self;

    /// Returns true if this number lies between 0 and 1.
    fn is_decimal(&self) -> bool {
        *self >= Self::zero() && *self <= Self::one()
    }

    /// Returns true if this number is a float between 0 and 1.
    fn clamp_01(&self) -> Self {
        self.max(Self::zero()).min(Self::one())
    }

    /// Clamps this number between the given bounds.
    fn clamp(&self, range: Range<Self>) -> Self {
        self.max(range.start).min(range.end)
    }

    /// Lerps from self to other by the set amount.
    fn lerp(&self, b: Self, t: Self) -> Self {
        *self + (b - *self) * t
    }

    /// Clamped lerp from self to other by the set amount.
    fn lerp_clamped(&self, b: Self, t: Self) -> Self {
        *self + (b - *self) * t.clamp_01()
    }
}

/// A strict subset of intiger types that are unsigned.
pub trait PrimaUInt: PrimaInt + Unsigned {}

impl PrimaNum for i128 {
    const BITS: usize = 128;
}
impl PrimaNum for i64 {
    const BITS: usize = 64;
}
impl PrimaNum for i32 {
    const BITS: usize = 32;
}
impl PrimaNum for i16 {
    const BITS: usize = 16;
}
impl PrimaNum for i8 {
    const BITS: usize = 8;
}
impl PrimaNum for f64 {
    const BITS: usize = 64;
}
impl PrimaNum for f32 {
    const BITS: usize = 32;
}
impl PrimaNum for u128 {
    const BITS: usize = 128;
}
impl PrimaNum for u64 {
    const BITS: usize = 64;
}
impl PrimaNum for u32 {
    const BITS: usize = 32;
}
impl PrimaNum for u16 {
    const BITS: usize = 16;
}
impl PrimaNum for u8 {
    const BITS: usize = 8;
}
impl PrimaNum for usize {
    const BITS: usize = 64;
}
impl PrimaNum for isize {
    const BITS: usize = 64;
}

impl PrimaInt for i128 {}
impl PrimaInt for i64 {}
impl PrimaInt for i32 {}
impl PrimaInt for i16 {}
impl PrimaInt for i8 {}
impl PrimaInt for u128 {}
impl PrimaInt for u64 {}
impl PrimaInt for u32 {}
impl PrimaInt for u16 {}
impl PrimaInt for u8 {}
impl PrimaInt for usize {}
impl PrimaInt for isize {}

impl PrimaFloat for f64 {
    fn pi() -> Self {
        f64::consts::PI
    }
}
impl PrimaFloat for f32 {
    fn pi() -> Self {
        f32::consts::PI
    }
}

impl PrimaUInt for usize {}
impl PrimaUInt for u8 {}
impl PrimaUInt for u16 {}
impl PrimaUInt for u32 {}
impl PrimaUInt for u64 {}
impl PrimaUInt for u128 {}

impl Cross<Point<f32>> for f32 {
    type Product = Point<f32>;

    fn cross(&self, other: &Point<f32>) -> Self::Product {
        Point {
            x: -self * other.y,
            y: other.x * *self,
        }
    }
}

impl Cross<Point<f64>> for f64 {
    type Product = Point<f64>;

    fn cross(&self, other: &Point<f64>) -> Self::Product {
        Point {
            x: -self * other.y,
            y: other.x * *self,
        }
    }
}

impl Cross<Vector<f32>> for f32 {
    type Product = Vector<f32>;

    fn cross(&self, other: &Vector<f32>) -> Self::Product {
        Vector {
            x: -self * other.y,
            y: other.x * *self,
        }
    }
}

impl Cross<Vector<f64>> for f64 {
    type Product = Vector<f64>;

    fn cross(&self, other: &Vector<f64>) -> Self::Product {
        Vector {
            x: -self * other.y,
            y: other.x * *self,
        }
    }
}
