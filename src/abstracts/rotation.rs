use std::ops::{Add, Div, Mul, Sub};

use crate::PrimaFloat;

/// A rotation in 2D space. Default rotation is in radians, WITHOUT the value of pi.
/// This allows for slightly more accurate conversions between radians and degrees,
/// and leaves the user to deal with the value of pi. 
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Rotation<N = f32>(N)
where
    N: PrimaFloat;

impl<N> Rotation<N>
where
    N: PrimaFloat,
{
    /// Creates a new rotation from radians. This is a value between -2 and 2 (so no pi please).
    pub fn new(angle: N) -> Self {
        Self(clamp(angle))
    }

    /// Creates a new rotation from radians. This is a value between -2 and 2 (so no pi please).
    pub fn from_radians(rads: N) -> Self {
        Self::new(rads)
    }

    /// Use this when passing in a literal value for radians.
    pub fn from_radians_raw(rads: N) -> Self {
        Self::from_radians(rads / N::pi())
    }

    /// Creates a new rotation from degrees.
    pub fn from_degrees(degs: N) -> Self {
        Self::new(degs / N::from_u8(180).unwrap())
    }

    /// Returns the rotation in degrees.
    pub fn as_degrees(&self) -> N {
        self.0 * N::from_u8(180).unwrap()
    }

    /// Returns the rotation in radians.
    pub fn as_rads(&self) -> N {
        self.0
    }

    /// Returns the rotation in radians divied by pi.
    pub fn as_rads_raw(&self) -> N {
        self.0 * N::pi()
    }

    /// Returns the opposite rotation.
    pub fn opposite(self) -> Self {
        Self::new(self.0 + N::pi())
    }
}

impl<N> Add<Rotation<N>> for Rotation<N>
where
    N: PrimaFloat,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.0 + rhs.0)
    }
}

impl<N> Sub<Rotation<N>> for Rotation<N>
where
    N: PrimaFloat,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.0 - rhs.0)
    }
}

impl<N> Mul<N> for Rotation<N>
where
    N: PrimaFloat,
{
    type Output = Self;

    fn mul(self, rhs: N) -> Self {
        Self::new(self.0 * rhs)
    }
}

impl<N> Div<N> for Rotation<N>
where
    N: PrimaFloat,
{
    type Output = Self;

    fn div(self, rhs: N) -> Self {
        Self::new(self.0 / rhs)
    }
}

fn clamp<N>(a: N) -> N
where
    N: PrimaFloat,
{
    let b = N::one() + N::one();
    ((a % b) + b) % b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotation_new() {
        let r1 = Rotation::from_degrees(180.0);
        assert_eq!(r1.as_rads_raw(), f32::pi());
        let r2 = Rotation::from_radians(1.5);
        assert_eq!(r2.as_degrees(), 270.0);
        let r3 = r1 + r2;
        assert_eq!(r3.as_rads(), 0.5);
        assert_eq!(r3.as_degrees(), 90.0);
        let r4 = r3 - Rotation::from_radians(0.75);
        assert_eq!(r4, Rotation::from_degrees(315.0));
    }
}
