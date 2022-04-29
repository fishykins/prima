use crate::{Mat2, PrimaFloat};
use std::ops::{Add, Div, Mul, Sub, Neg};

macro_rules! rotation_impl(
    ($T: ident) => {
        impl<N> $T<N> where N: PrimaFloat {
            /// Creates a new rotation from radians. Same as `new`. just explicit.
            pub fn from_radians(rads: N) -> Self {
                Self::new(rads)
            }

            /// Use this when passing in a literal value for radians.
            pub fn from_radians_raw(rads: N) -> Self {
                Self::from_radians(rads / N::pi())
            }

            /// Creates a new rotation at 0 radians.
            pub fn zero() -> Self {
                Self::new(N::zero())
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

            /// Returns the angle in radians divied by pi.
            pub fn as_rads_raw(&self) -> N {
                self.0 * N::pi()
            }

            /// Returns true if clockwise.
            pub fn clockwise(&self) -> bool {
                self.0 > N::zero()
            }

            /// Returns true if counter-clockwise.
            pub fn counter_clockwise(&self) -> bool {
                self.0 < N::zero()
            }

            /// Returns true if the rotation is zero.
            pub fn is_none(&self) -> bool {
                self.0 == N::zero()
            }

            /// Returns the sine of the angle.
            pub fn sin(&self) -> N {
                self.as_rads_raw().sin()
            }

            /// Returns the co-sine of the angle.
            pub fn cos(&self) -> N {
                self.as_rads_raw().cos()
            }

            /// Returns the tangent of the angle.
            pub fn tan(&self) -> N {
                self.as_rads_raw().tan()
            }

            /// Convert to a rotation matrix.
            pub fn to_matrix(self) -> Mat2<N> {
                self.into()
            }

            /// Mirrors the rotation.
            pub fn mirror(self) -> Self {
                Self::new(N::zero() - self.as_rads())
            }
        }

        impl<N> Add for $T<N>
        where
            N: PrimaFloat,
        {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self::new(self.0 + rhs.0)
            }
        }

        impl<N> Sub for $T<N>
        where
            N: PrimaFloat,
        {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self::new(self.0 - rhs.0)
            }
        }

        impl<N> Neg for $T<N>
        where
            N: PrimaFloat,
        {
            type Output = Self;

            fn neg(self) -> Self {
                Self::new(N::zero() - self.0)
            }
        }

        impl<N> Mul<N> for $T<N>
        where
            N: PrimaFloat,
        {
            type Output = Self;

            fn mul(self, rhs: N) -> Self {
                Self::new(self.0 * rhs)
            }
        }

        impl<N> Div<N> for $T<N>
        where
            N: PrimaFloat,
        {
            type Output = Self;

            fn div(self, rhs: N) -> Self {
                Self::new(self.0 / rhs)
            }
        }

        impl<N> Into<Mat2<N>> for $T<N>
        where
            N: PrimaFloat,
        {
            fn into(self) -> Mat2<N> {
                let two = N::one() + N::one();
                let half = N::one() / two;

                let r = -self.as_rads();

                let (c, s) = if r % half == N::zero() {
                    if r % two == N::zero() {
                        (N::one(), N::zero())
                    } else if r % N::one() == N::zero() {
                        (N::zero() -N::one(), N::zero())
                    } else if r % (N::one() + half) == N::zero() {
                        (N::zero(), N::one())
                    } else {
                        (N::zero(), N::zero() -N::one())
                    }
                } else {
                    let r = -self.as_rads_raw();
                    (r.cos(), r.sin())
                };

                let mat = Mat2::<N>::new(c, -s, s, c);
                mat
            }
        }
    }
);

/// A rotation to be applied to a point in the clockwise rotation. Similar to [Angle], but un-clamped.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Rotation<N = f32>(N)
where
    N: PrimaFloat;

/// An angle in 2D space. Default angle is in radians, WITHOUT the value of pi.
/// This allows for slightly more accurate conversions between radians and degrees,
/// and leaves the user to deal with the value of pi.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Angle<N = f32>(N)
where
    N: PrimaFloat;

rotation_impl!(Rotation);
rotation_impl!(Angle);

impl<N> Rotation<N>
where
    N: PrimaFloat,
{
    /// Creates a new rotation from radians, without the value of pi.
    pub fn new(r: N) -> Self {
        Self(r)
    }
}

impl<N> Angle<N>
where
    N: PrimaFloat,
{
    /// Creates a new angle from radians. This is a value between 0 and 2 (so no pi please).
    pub fn new(angle: N) -> Self {
        Self(clamp(angle))
    }

    /// Returns the opposite angle.
    pub fn opposite(self) -> Self {
        Self::new(self.0 + N::pi())
    }
}

impl<N> From<Rotation<N>> for Angle<N>
where
    N: PrimaFloat,
{
    fn from(r: Rotation<N>) -> Self {
        Self::new(r.0)
    }
}

impl<N> From<Angle<N>> for Rotation<N>
where
    N: PrimaFloat,
{
    fn from(a: Angle<N>) -> Self {
        Self::new(a.0)
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
    use crate::Point;

    #[test]
    fn test_angle_new() {
        let r1 = Angle::from_degrees(180.0f32);
        assert_eq!(r1.as_rads_raw(), f32::pi());
        let r2 = Angle::from_radians(1.5f32);
        assert_eq!(r2.as_degrees(), 270.0);
        let r3 = r1 + r2;
        assert_eq!(r3.as_rads(), 0.5);
        assert_eq!(r3.as_degrees(), 90.0);
        let r4 = r3 - Angle::from_radians(0.75f32);
        assert_eq!(r4, Angle::from_degrees(315.0f32));
    }

    #[test]
    fn rotate_point_test() {
        let p = Point::new(0.0, 10.0);
        let r = Rotation::from_degrees(90.0);
        let p2 = p.rotate(r);
        assert_eq!(p2, Point::new(10.0, 0.0));

        let r2 = Rotation::from_degrees(-180.0);
        let p3 = p2.rotate(r2);
        assert_eq!(p3, Point::new(-10.0, 0.0));
    }
}
