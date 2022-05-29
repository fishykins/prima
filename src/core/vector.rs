use serde::{Deserialize, Serialize};

use crate::{
    core::{Angle, AngleMat, Point, Rotation},
    nums::{PrimaFloat, PrimaNum},
    traits::{Cross, Magnitude},
    xy_impl,
};
use std::ops::{AddAssign, Neg, Sub, SubAssign};

/// A vector in 2 dimensions.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Vector<N> {
    /// The vector's x component.
    pub x: N,
    /// The vector's y component.
    pub y: N,
}

xy_impl!(Vector);

impl<N> Vector<N>
where
    N: PrimaNum,
{
    /// Returns the counter-clockwise perpendicular vector.
    pub fn perpendicular_cc(self) -> Self {
        Vector {
            x: N::zero() - self.y,
            y: self.x,
        }
    }

    /// Returns the clockwise perpendicular vector.
    pub fn perpendicular(self) -> Self {
        Vector {
            x: self.y,
            y: N::zero() - self.x,
        }
    }

    /// Returns the inverted vector.
    pub fn inverted(self) -> Self {
        Vector {
            x: N::zero() - self.x,
            y: N::zero() - self.y,
        }
    }

    /// Converts the vector to a point.
    pub fn as_point(&self) -> Point<N> {
        Point::new(self.x, self.y)
    }

    /// Returns an up vector.
    pub fn up() -> Self {
        Vector {
            x: N::zero(),
            y: N::one(),
        }
    }

    /// Returns a down vector.
    pub fn down() -> Self {
        Vector {
            x: N::zero(),
            y: N::zero() - N::one(),
        }
    }

    /// Returns a left vector.
    pub fn left() -> Self {
        Vector {
            x: N::zero() - N::one(),
            y: N::zero(),
        }
    }

    /// Returns a right vector.
    pub fn right() -> Self {
        Vector {
            x: N::one(),
            y: N::zero(),
        }
    }
}

impl<N> Vector<N>
where
    N: PrimaFloat,
{
    /// Normalize the vector.
    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    /// Lerps from self to other.
    pub fn lerp(&self, other: Self, t: N) -> Self {
        let mag = self.magnitude();
        let mag_other = other.magnitude();
        let mag_lerp = mag.lerp(mag_other, t);
        let mag_lerp_inv = N::one() / mag_lerp;
        Self {
            x: self.x * mag_lerp_inv,
            y: self.y * mag_lerp_inv,
        }
    }

    /// Converts the vector to a rotation.
    pub fn as_rotation(&self) -> Rotation<N> {
        if self.x == N::zero() {
            if self.y >= N::zero() {
                Rotation::new(N::zero())
            } else {
                Rotation::new(N::one())
            }
        } else {
            if self.y == N::zero() {
                let half = N::one() / (N::one() + N::one());
                if self.x > N::zero() {
                    Rotation::new(half)
                } else {
                    Rotation::new(N::one() + half)
                }
            } else {
                let r = self.x / self.y;
                let r = r.atan();
                Rotation::from_radians_pi(r)
            }
        }
    }

    /// Converts the vector to an angle.
    pub fn as_angle(&self) -> Angle<N> {
        self.as_rotation().into()
    }

    /// Gets the rotation from self to other.
    pub fn angle_of_difference(&self, other: &Self) -> Angle<N> {
        let d = self.dot(other);
        let cos_theta = d / (self.magnitude() * other.magnitude());
        let theta = cos_theta.acos();
        Angle::from_radians_pi(theta)
    }

    /// Gets the rotation from self to other.
    pub fn rotation_to(&self, other: &Self) -> Rotation<N> {
        self.as_angle().rotation_to(&other.as_angle())
    }
}

impl<N> Magnitude<N> for Vector<N>
where
    N: PrimaFloat,
{
    fn magnitude_squared(&self) -> N {
        self.x * self.x + self.y * self.y
    }
}

impl<N> Add for Vector<N>
where
    N: PrimaNum,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<N> AddAssign for Vector<N>
where
    N: PrimaNum + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<N> Sub for Vector<N>
where
    N: PrimaNum,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<N> SubAssign for Vector<N>
where
    N: PrimaNum + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
