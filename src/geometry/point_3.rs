use std::fmt::Display;

use crate::xyz_ops_impl;

/// A three dimensional point.
#[derive(Copy, Clone, Debug)]
pub struct Point3<N> {
    /// The x coordinate of the point.
    pub x: N,
    /// The y coordinate of the point.
    pub y: N,
    /// The z coordinate of the point.
    pub z: N,
}

xyz_ops_impl!(Point3);

impl<N> PartialEq for Point3<N>
where
    N: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Point3<N>) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<N> Display for Point3<N>
where
    N: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
