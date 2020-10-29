use vek::Vec3;
use crate::core::{Point, GeoNum};
use std::ops::{Mul, Div};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vertex<T> where T: GeoNum {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vertex<T> where T: GeoNum {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z
        }
    }

    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn from_tripple(pos: (T, T, T)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            z: pos.2
        }
    }

    pub fn from_quad(pos: (T, T, T, T)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            z: pos.2
        }
    }
}

impl<T> Point<T> for Vertex<T> where T: GeoNum  {
    fn from_vec3(pos: Vec3<T>) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            z: pos.z
        }
    }

    fn to_vec3(self) -> Vec3<T> {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl<T> Mul for Vertex<T> where T: GeoNum {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.z;
        Self::new(x, y, z)
    }
}

impl<T> Mul<T> for Vertex<T> where T: GeoNum {
    type Output = Vertex<T>;

    fn mul(self, rhs: T) -> Self {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        Self::new(x, y, z)
    }
}

impl<T> Div<T> for Vertex<T> where T: GeoNum {
    type Output = Vertex<T>;

    fn div(self, rhs: T) -> Self {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        Self::new(x, y, z)
    }
}