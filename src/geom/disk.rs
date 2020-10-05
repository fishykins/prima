use super::Triangle;
use crate::core::OrdNum;
use vek::{Vec2, Disk};
use num::{Signed, Float};

pub trait DiskExt<T> where T: OrdNum + Signed + Float, Self: std::marker::Sized {
    fn from_triangle(triangle: Triangle<T>) -> Option<Self>;
}

impl<T> DiskExt<T> for Disk<T,T> where T: OrdNum + Signed + Float, Self: std::marker::Sized {
    fn from_triangle(triangle: Triangle<T>) -> Option<Self> {
        let (p1, p2, p3) = triangle.to_tripple();
        let minus2 = T::zero() - T::one() - T::one();
        
        let c1 = p3.x * p3.x + p3.y * p3.y - p1.x * p1.x - p1.y * p1.y;
        let c2 = p3.x * p3.x + p3.y * p3.y - p2.x * p2.x - p2.y * p2.y;
        let a1 = minus2 * (p1.x - p3.x);
        let a2 = minus2 * (p2.x - p3.x);
        let b1 = minus2 * (p1.y - p3.y);
        let b2 = minus2 * (p2.y - p3.y);

        let numer = c1 * a2 - c2 * a1;
        let denom = b1 * a2 - b2 * a1;

        if denom == T::zero() {
            return None;
        }
        let y_cen = numer / denom;

        let x_cen = if a2 != T::zero() {
            (c2 - b2 * y_cen) / a2
        } else {
            (c1 - b1 * y_cen) / a1
        };

        let center = Vec2::<T>::new(x_cen, y_cen);
        let radius = center.distance(p1);
        Some(Self {
            center,
            radius,
        })
    }
}