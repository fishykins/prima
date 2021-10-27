
use num::{Num, FromPrimitive, ToPrimitive};
use std::fmt::{Debug, Display};
use std::{f32,f64};

/// A trait implimenting all the nesciasry traits to ensure that a generic number will be usable in most cases
pub trait OrdNum: Num + PartialOrd + Clone + Copy + Display + Debug + FromPrimitive + ToPrimitive {}

impl OrdNum for i64 {}
impl OrdNum for i32 {}
impl OrdNum for i8 {}
impl OrdNum for f64 {}
impl OrdNum for f32 {}
impl OrdNum for u64 {}
impl OrdNum for u32 {}
impl OrdNum for u8 {}
impl OrdNum for usize {}