use num_traits::Num;

use crate::{Shape2, Collide};

/// A Rectangle in 2D space. Alternative to Aabr. 
#[derive(Debug, Clone, Default)]
pub struct Rect<N> {
    /// The minimum x point of the rectangle.
    pub x: N,
    /// The minimum y point of the rectangle.
    pub y: N,
    /// The width of the rectangle.
    pub w: N,
    /// The height of the rectangle.
    pub h: N,
}

impl<N> Rect<N> where N: Num + PartialOrd + Copy {
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
}

impl<N> Shape2<N> for Rect<N> where N: Num + PartialOrd + Copy + PartialOrd {
    fn area(&self) -> N {
        self.w * self.h
    }

    fn circumference(&self) -> N {
        self.w + self.w + self.h + self.h
    }

    fn center(&self) -> crate::Point2<N> {
        let two = N::one() + N::one();
        crate::Point2::new(self.x + self.w / two, self.y + self.h / two)
    }

    fn bounding_box(&self) -> crate::Aabr<N> {
        self.clone().into()
    }

    fn contains_point(&self, point: &crate::Point2<N>) -> bool {
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

impl<N> Collide for Rect<N> where N: Num + Copy + PartialOrd {
    type Output = Rect<N>;

    fn collision(&self, other: &Self) -> Option<Self::Output> {
        let x_min = if self.x > other.x { self.x } else { other.x };
        let y_min = if self.y > other.y { self.y } else { other.y };

        let x_max = if self.x + self.w < other.x + other.w { self.x + self.w } else { other.x + other.w };
        let y_max = if self.y + self.h < other.y + other.h { self.y + self.h } else { other.y + other.h };

        if x_min <= x_max && y_min <= y_max {
            Some(Rect::new(x_min, y_min, x_max - x_min, y_max - y_min))
        } else {
            None
        }
    }
}