use vek::Vec3;
use super::{IndexType, GeoNum};

pub trait Point<T> where T: GeoNum {
    fn from_vec3(pos: Vec3<T>) -> Self;
    fn to_vec3(self) -> Vec3<T>;
}

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct PointIndex<Ix = crate::core::DefaultIx>(Ix);

impl<Ix: IndexType> PointIndex<Ix> {
    #[inline]
    pub fn new(x: usize) -> Self {
        PointIndex(IndexType::new(x))
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0.index()
    }

    #[inline]
    pub fn end() -> Self {
        PointIndex(IndexType::max())
    }
}