/// Generates generic implementations for rotation based structs.
#[macro_export]
macro_rules! rotation_impl(
    ($T: ident) => {
        impl<N> $T<N> where N: PrimaFloat {
            /// Creates a new rotation at 0 radians.
            pub fn zero() -> Self {
                Self::new(N::zero())
            }

            /// Creates a new rotation from radians. Same as `new`. just explicit.
            pub fn from_radians(rads: N) -> Self {
                Self::new(rads)
            }

            /// Use this when passing in a literal value for radians.
            pub fn from_radians_pi(rads: N) -> Self {
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
            pub fn as_radians(&self) -> N {
                self.0
            }

            /// Returns the angle in radians divied by pi.
            pub fn as_radians_pi(&self) -> N {
                self.0 * N::pi()
            }

            /// Returns the angle matrix for this rotation.
            pub fn to_matrix(self) -> AngleMat<N> {
                self.into()
            }

            /// Returns true if the rotation is zero.
            pub fn is_none(&self) -> bool {
                self.0 == N::zero()
            }

            /// Returns true if the rotation is aligned with the x or y axis.
            /// This essentially checks if facing up, down, left or right.
            pub fn is_axis_aligned(&self) -> bool {
                self.0 % (N::one() / (N::one() + N::one())) == N::zero()
            }

            /// Returns the sine of the angle.
            pub fn sin(&self) -> N {
                self.as_radians_pi().sin()
            }

            /// Returns the co-sine of the angle.
            pub fn cos(&self) -> N {
                self.as_radians_pi().cos()
            }

            /// Returns the tangent of the angle.
            pub fn tan(&self) -> N {
                self.as_radians_pi().tan()
            }

            /// Mirrors the rotation.
            pub fn mirror(self) -> Self {
                Self::new(N::zero() - self.as_radians())
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

        impl<N> AddAssign for $T<N>
        where
            N: PrimaFloat,
        {
            fn add_assign(&mut self, rhs: Self) {
                self.0 = Self::new(self.0 + rhs.0).0;
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

        impl<N> SubAssign for $T<N>
        where
            N: PrimaFloat,
        {
            fn sub_assign(&mut self, rhs: Self) {
                self.0 = self.0 - rhs.0;
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

        impl<N> MulAssign<N> for $T<N>
        where
            N: PrimaFloat,
        {
            fn mul_assign(&mut self, rhs: N) {
                self.0 = Self::new(self.0 * rhs).0;
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

        impl<N> DivAssign<N> for $T<N>
        where
            N: PrimaFloat,
        {
            fn div_assign(&mut self, rhs: N) {
                self.0 = Self::new(self.0 / rhs).0;
            }
        }

        impl<N> Into<AngleMat<N>> for $T<N>
        where
            N: PrimaFloat,
        {
            fn into(self) -> AngleMat<N> {
                let two = N::one() + N::one();
                let half = N::one() / two;

                let r = -self.as_radians();

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
                    let r = -self.as_radians_pi();
                    (r.cos(), r.sin())
                };

                let mat = AngleMat::<N>::new(c, -s, s, c);
                mat
            }
        }
    }
);

//=============================================================================//
//=============================================================================//

/// Implements generic opperations for xy based structs.
#[macro_export]
macro_rules! xy_impl(
    ($T: ident) => {
        use std::ops::{Add, Div, Mul};

        impl<N> $T<N> where N: PrimaNum {
            /// Creates a new point.
            pub fn new(x: N, y: N) -> Self {
                $T { x, y }
            }
            /// Creates point from a single value.
            pub fn splat(n: N) -> Self {
                $T { x: n, y: n }
            }
            /// Creates a point at zero.
            pub fn zero() -> Self {
                Self::splat(N::zero())
            }
            /// Creates a point at one.
            pub fn one() -> Self {
                Self::splat(N::one())
            }

            /// Returns the dot product of two points.
            pub fn dot(&self, other: &Self) -> N {
                self.x * other.x + self.y * other.y
            }
        }

        impl<N> $T<N> where N: PrimaFloat {
            /// Rotates the point around the given point by the given angle.
            pub fn rotate_around(&self, point: Point<N>, rotation: Rotation<N>) -> Self {
                let v = Vector::new(self.x - point.x, self.y - point.y);
                let rv = v * rotation;
                let p = point + rv;
                Self::new(p.x, p.y)
            }

            /// Rotates the point around the given point by the given angle.
            pub fn rotate_around_mat(&self, point: Point<N>, rotation: AngleMat<N>) -> Self {
                let v = Vector::new(self.x - point.x, self.y - point.y);
                let rv = v * rotation;
                let p = point + rv;
                Self::new(p.x, p.y)
            }
        }

        impl<N> Neg for $T<N> where N: PrimaNum + Neg<Output = N> {
            type Output = Self;

            fn neg(self) -> Self {
                Self::new(-self.x, -self.y)
            }
        }

        impl<N> Mul<N> for $T<N> where N: PrimaFloat {
            type Output = Self;

            fn mul(self, rhs: N) -> Self {
                Self::new(self.x * rhs, self.y * rhs)
            }
        }

        impl<N> Div<N> for $T<N> where N: PrimaFloat {
            type Output = Self;

            fn div(self, rhs: N) -> Self {
                Self::new(self.x / rhs, self.y / rhs)
            }
        }

        impl<N> Mul<AngleMat<N>> for $T<N>
        where N: Mul<Output = N> + Add<Output = N> + Copy {
            type Output = Self;

            fn mul(self, other: AngleMat<N>) -> Self {
                Self {
                    x: self.x * other.m00 + self.y * other.m10,
                    y: self.x * other.m01 + self.y * other.m11,
                }
            }
        }

        impl<N> Mul<Rotation<N>> for $T<N>
        where N: Mul<Output = N> + Add<Output = N> + PrimaFloat {
            type Output = Self;

            fn mul(self, other: Rotation<N>) -> Self {
                self * other.to_matrix()
            }
        }

        impl<N> Cross for $T<N> where N: PrimaNum {
            type Product = N;

            fn cross(&self, other: &Self) -> Self::Product {
                self.x * other.y - self.y * other.x
            }
        }

        impl<N> Cross<N> for $T<N> where N: PrimaNum {
            type Product = Self;

            fn cross(&self, other: &N) -> Self::Product {
                Self {
                    x: self.y * *other,
                    y: N::zero() -self.x * *other,
                }
            }
        }

        impl<N> Into<(N, N)> for $T<N> where N: PrimaNum {
            fn into(self) -> (N, N) {
                (self.x, self.y)
            }
        }
    }
);
