use serde::{Deserialize, Serialize};

use crate::{Shape2, PrimaNum, Point};

/// A Rectangle in 2D space. Alternative to Aabr. 
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rect<N = super::DefaultFloat> {
    /// The minimum x point of the rectangle.
    pub x: N,
    /// The minimum y point of the rectangle.
    pub y: N,
    /// The width of the rectangle.
    pub w: N,
    /// The height of the rectangle.
    pub h: N,
}

impl<N> Rect<N> where N: PrimaNum {
    /// Constructs a new rectangle.
    pub fn new(x: N, y: N, w: N, h: N) -> Self {
        Self { x, y, w, h }
    }

    /// Checks if the rectangle is valid (i.e. has a positive width and height) and is not equal to zero.
    /// 
    /// Due to the nature of a rectangle, being 'valid' is not something that can (or should) be enforced.
    /// To enact a validated rectangle, consider converting it into an Aabr.
    pub fn is_valid(&self) -> bool {
        self.w > N::zero() && self.h > N::zero()
    }

    /// Returns four [Rects] that are the result of splitting the current rectangle into four equal parts.
    pub fn into_quad(self) -> [Self; 4] {
        let two = N::one() + N::one();
        let w = self.w / two;
        let h = self.h / two;
        [
            Self::new(self.x, self.y, w, h),
            Self::new(self.x + w, self.y, w, h),
            Self::new(self.x + w, self.y + h, w, h),
            Self::new(self.x, self.y + h, w, h),
        ]
    }

    /// Returns the minimum point of the [Rect]. 
    pub fn min(&self) -> Point<N> {
        Point::new(self.x, self.y)
    }

    /// Returns the maximum point of the [Rect].
    pub fn max(&self) -> Point<N> {
        Point::new(self.x + self.w, self.y + self.h)
    }
}

impl<N> Shape2<N> for Rect<N> where N: PrimaNum {
    fn area(&self) -> N {
        self.w * self.h
    }

    fn circumference(&self) -> N {
        self.w + self.w + self.h + self.h
    }

    fn center(&self) -> crate::Point<N> {
        let two = N::one() + N::one();
        crate::Point::new(self.x + self.w / two, self.y + self.h / two)
    }

    fn bounding_box(&self) -> crate::Aabr<N> {
        self.clone().into()
    }

    fn contains_point(&self, point: &crate::Point<N>) -> bool {
        let x_ok = if self.w >= N::zero() {
            point.x >= self.x && point.x <= self.x + self.w
        } else {
            point.x <= self.x && point.x >= self.x + self.w
        };
        let y_ok = if self.h >= N::zero() {
            point.y >= self.y && point.y <= self.y + self.h
        } else {
            point.y <= self.y && point.y >= self.y + self.h
        };
        x_ok && y_ok
    }
}