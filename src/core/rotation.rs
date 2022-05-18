use crate::{
    nums::{PrimaFloat, PrimaNum},
    rotation_impl,
};
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A rotation is the equivalent to an unbound [Angle].
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Rotation<N>(N);

/// An angle is bound to the range of [0..2].
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Angle<N>(N);

/// A 2x2 angular matrix.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AngleMat<N = f32> {
    /// Top left
    pub m00: N,
    /// Top right
    pub m10: N,
    /// Bottom left
    pub m01: N,
    /// Bottom right
    pub m11: N,
}

//=============================================================//
//======================= IMPLEMENTATIONS =====================//
//=============================================================//

impl<N> Rotation<N>
where
    N: PrimaNum,
{
    /// Creates a new angle from a radian value.
    pub fn new(a: N) -> Self {
        Rotation(a)
    }

    /// Returns true if clockwise.
    pub fn clockwise(&self) -> bool {
        self.0 > N::zero()
    }

    /// Returns true if counter-clockwise.
    pub fn counter_clockwise(&self) -> bool {
        self.0 < N::zero()
    }
}

impl<N> Angle<N> {
    /// Creates a new angle from a radian value.
    pub fn new(a: N) -> Self {
        Angle(a)
    }
}

rotation_impl!(Rotation);
rotation_impl!(Angle);

impl<N> AngleMat<N> {
    /// Creates a new matrix.
    pub fn new(m00: N, m10: N, m01: N, m11: N) -> Self {
        Self { m00, m01, m10, m11 }
    }
}

//=============================================================//
//========================= CONVERSIONS =======================//
//=============================================================//

/// Clamps a value to the range of [0..2], applying overflow.
pub fn clamp_radians<N>(a: N) -> N
where
    N: PrimaFloat,
{
    let b = N::one() + N::one();
    ((a % b) + b) % b
}

impl<N> From<Rotation<N>> for Angle<N>
where
    N: PrimaFloat,
{
    fn from(r: Rotation<N>) -> Self {
        Angle(clamp_radians(r.0))
    }
}

impl<N> From<Angle<N>> for Rotation<N> {
    fn from(a: Angle<N>) -> Self {
        Rotation(a.0)
    }
}

//=============================================================//
//======================== OPPERATIONS ========================//
//=============================================================//
impl<N> AddAssign<Rotation<N>> for Angle<N>
where
    N: PrimaFloat,
{
    fn add_assign(&mut self, rhs: Rotation<N>) {
        *self = Angle(clamp_radians(self.0 + rhs.0));
    }
}

impl<N> Add<Rotation<N>> for Angle<N>
where
    N: PrimaFloat,
{
    type Output = Self;

    fn add(self, rhs: Rotation<N>) -> Self {
        Angle(clamp_radians(self.0 + rhs.0))
    }
}

impl<N> SubAssign<Rotation<N>> for Angle<N>
where
    N: PrimaFloat,
{
    fn sub_assign(&mut self, rhs: Rotation<N>) {
        *self = Angle(clamp_radians(self.0 - rhs.0));
    }
}

impl<N> Sub<Rotation<N>> for Angle<N>
where
    N: PrimaFloat,
{
    type Output = Self;

    fn sub(self, rhs: Rotation<N>) -> Self {
        Angle(clamp_radians(self.0 - rhs.0))
    }
}
