use num_traits::{FromPrimitive, Num, ToPrimitive, Unsigned, Float};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::{f32, f64};

/// A more general trait that covers most number types.
pub trait PrimaNum:
    Num + PartialOrd + Clone + Copy + Display + Debug + FromPrimitive + ToPrimitive
{
}

/// Intiger types.
pub trait PrimaInt: PrimaNum + Hash {}

/// Float types.
pub trait PrimaFloat: PrimaNum + Float {}

/// A strict subset of intiger types that are unsigned.
pub trait PrimaUInt: PrimaInt + Unsigned {}

impl PrimaNum for i128 {}
impl PrimaNum for i64 {}
impl PrimaNum for i32 {}
impl PrimaNum for i16 {}
impl PrimaNum for i8 {}
impl PrimaNum for f64 {}
impl PrimaNum for f32 {}
impl PrimaNum for u128 {}
impl PrimaNum for u64 {}
impl PrimaNum for u32 {}
impl PrimaNum for u16 {}
impl PrimaNum for u8 {}
impl PrimaNum for usize {}
impl PrimaNum for isize {}

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

impl PrimaFloat for f64 {}
impl PrimaFloat for f32 {}

impl PrimaUInt for usize {}
impl PrimaUInt for u8 {}
impl PrimaUInt for u16 {}
impl PrimaUInt for u32 {}
impl PrimaUInt for u64 {}
impl PrimaUInt for u128 {}
