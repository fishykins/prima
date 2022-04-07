/// Implements the typical traits required for an XYZ based struct.
#[macro_export]
macro_rules! xyz_ops_impl(
    ($T: ident) => {
        use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
        use crate::Coordinate;

        impl<N> $T<N> where N: Copy {
            /// Creates a new point.
            pub fn new(x: N, y: N, z: N) -> Self {
                $T { x, y, z }
            }

            /// Creates point from a single value.
            pub fn splat(n: N) -> Self {
                $T { x: n, y: n, z: n }
            }
        }

        impl<N> Coordinate<N> for $T<N> where N: num_traits::Num + Copy {
            fn axis(&self) -> crate::AxisValue<N> {
                crate::base::AxisValue::XYZ(self.x, self.y, self.z)
            }
        }

        impl<N> $T<N> where N: num_traits::Float {
            /// Returns zero vector.
            pub fn zero() -> Self {
                $T { x: N::zero(), y: N::zero(), z: N::zero() }
            }
            /// Returns one vector.
            pub fn one() -> Self {
                $T { x: N::one(), y: N::one(), z: N::one() }
            }
        }

        impl<N> Into<(N, N, N)> for $T<N> {
            fn into(self) -> (N, N, N) {
                (self.x, self.y, self.z)
            }
        }

        impl<N> From<(N, N, N)> for $T<N> {
            fn from((x, y, z): (N, N, N)) -> Self {
                Self {
                    x,
                    y,
                    z,
                }
            }
        }

        impl<N> From<Vec<N>> for $T<N> where N: Copy {
            fn from(v: Vec<N>) -> Self {
                Self {
                    x: v[0],
                    y: v[1],
                    z: v[2],
                }
            }
        }

        impl<N> Add for $T<N>
            where N: Add<Output = N> {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self {
                    x: self.x + other.x,
                    y: self.y + other.y,
                    z: self.z + other.z,
                }
            }
        }

        impl<N> Sub for $T<N>
            where N: Sub<Output = N> {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self {
                    x: self.x - other.x,
                    y: self.y - other.y,
                    z: self.z - other.z,
                }
            }
        }

        impl<N> Mul<N> for $T<N>
            where N: Mul<Output = N> + Copy {
            type Output = Self;

            fn mul(self, other: N) -> Self {
                Self {
                    x: self.x * other,
                    y: self.y * other,
                    z: self.z * other,
                }
            }
        }

        impl<N> Div<N> for $T<N>
            where N: Div<Output = N> + Copy {
            type Output = Self;

            fn div(self, other: N) -> Self {
                Self {
                    x: self.x / other,
                    y: self.y / other,
                    z: self.z / other,
                }
            }
        }

        impl<N> AddAssign for $T<N>
        where
            N: Add<Output = N> + AddAssign,
        {
            #[inline]
            fn add_assign(&mut self, other: $T<N>) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        impl<N> SubAssign for $T<N>
        where
            N: Sub<Output = N> + SubAssign,
        {
            #[inline]
            fn sub_assign(&mut self, other: $T<N>) {
                self.x -= other.x;
                self.y -= other.y;
                self.z -= other.z;
            }
        }
    }
);
