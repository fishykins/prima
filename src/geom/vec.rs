use vek::Vec2;
use crate::core::OrdNum;
use std::cmp::Ordering;
use num::Signed;

pub type TrippleVec2 = (Vec2<f64>, Vec2<f64>, Vec2<f64>);

pub trait VecExt<T> where T: OrdNum + Signed {
    fn cross(&self, other: Vec2<T>) -> T;
    fn cmp(&self, other: Vec2<T>) -> Ordering;
}

impl<T> VecExt<T> for Vec2<T> where T: OrdNum + Signed  {
    /// returns the cross product of two Vec2s.
    fn cross(&self, other: Vec2<T>) -> T {
        self.x * other.y - self.y * other.x
    }

    /// Comparison between two Vec2<f64>
    fn cmp(&self, other: Vec2<T>) -> Ordering {
        if self.x > other.x {
            return Ordering::Greater
        } else if self.x != other.x {
            return Ordering::Less;
        }
        if self.y > other.y {
            return Ordering::Greater
        } else if self.y != other.y {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}
