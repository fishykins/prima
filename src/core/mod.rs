mod direction;

pub use direction::Direction;

use num::{Num, CheckedMul, FromPrimitive, ToPrimitive, Signed, Integer, Float};
use std::fmt::{Debug, Display};
use std::ops::{SubAssign, AddAssign};
use std::{f32,f64};

pub type DefaultIx = usize;
pub trait OrdNum: Num + PartialOrd + Clone + Copy + Display + Debug + FromPrimitive + ToPrimitive {}
pub trait GridNum : OrdNum + Integer + Signed + AddAssign + SubAssign + CheckedMul {}
pub trait GeoNum : OrdNum + Float + Signed {
    fn pi() -> Self;
    fn rad() -> Self;
}

pub const F64_2_PI: f64 = f64::consts::PI * 2.;
pub const F32_2_PI: f32 = f32::consts::PI * 2.;


impl GridNum for i64 {}
impl GridNum for i32 {}
impl GridNum for i8 {}

impl GeoNum for f64 {
    fn pi() -> Self {
        f64::consts::PI
    }
    fn rad() -> Self {
        F64_2_PI
    }
}

impl GeoNum for f32 {
    fn pi() -> Self {
        f32::consts::PI
    }
    fn rad() -> Self {
        F32_2_PI
    }
}

impl OrdNum for i64 {}
impl OrdNum for i32 {}
impl OrdNum for i8 {}
impl OrdNum for f64 {}
impl OrdNum for f32 {}
impl OrdNum for u64 {}
impl OrdNum for u32 {}
impl OrdNum for u8 {}
impl OrdNum for usize {}