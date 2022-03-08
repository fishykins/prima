use std::{fmt::Display, ops::{Add, Sub, SubAssign, AddAssign}};

/// A three dimensional point.
#[derive(Copy, Clone, Debug)]
pub struct Point3<N> {
    /// The x coordinate of the point.
    pub x: N,
    /// The y coordinate of the point.
    pub y: N,
    /// The z coordinate of the point.
    pub z: N
}

impl<N> Point3<N> {
    /// Creates a new point.
    #[inline]
    pub fn new(x: N, y: N, z: N) -> Point3<N> {
        Point3 { x, y, z }
    }
}

impl<N> PartialEq for Point3<N>
    where N: PartialEq {
    #[inline]
    fn eq(&self, other: &Point3<N>) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<N> Display for Point3<N>
    where N: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<N> Add for Point3<N>
    where N: Add<Output=N> {
    type Output = Point3<N>;

    #[inline]
    fn add(self, other: Point3<N>) -> Point3<N> {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<N> Sub for Point3<N>
    where N: Sub<Output=N> {
    type Output = Point3<N>;

    #[inline]
    fn sub(self, other: Point3<N>) -> Point3<N> {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl<N> SubAssign for Point3<N>
    where N: Sub<Output=N> + SubAssign {
    #[inline]
    fn sub_assign(&mut self, other: Point3<N>) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<N> AddAssign for Point3<N>
    where N: Add<Output=N> + AddAssign {
    #[inline]
    fn add_assign(&mut self, other: Point3<N>) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<N> Into<(N, N, N)> for Point3<N> {
    #[inline]
    fn into(self) -> (N, N, N) {
        (self.x, self.y, self.z)
    }
}

impl<N> From<(N, N, N)> for Point3<N> {
    #[inline]
    fn from((x, y, z): (N, N, N)) -> Point3<N> {
        Point3::new(x, y, z)
    }
}