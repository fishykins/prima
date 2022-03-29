use crate::{Collide, Aabr, Rect, Point2, PrimaNum};

impl<N> Collide<Rect<N>> for Aabr<N> where N: PrimaNum {
    type Output = Aabr<N>;

    fn collision(&self, other: &Rect<N>) -> Option<Self::Output> {
        let other: Aabr<N> = Aabr::<N>::from(other.clone());
        self.collision(&other)
    }
}

impl<N> Collide<Aabr<N>> for Rect<N> where N: PrimaNum {
    type Output = Aabr<N>;

    fn collision(&self, other: &Aabr<N>) -> Option<Self::Output> {
        Aabr::<N>::from(self.clone()).collision(other)
    }
}

impl<N> Into<Rect<N>> for Aabr<N> where N: PrimaNum {
    fn into(self) -> Rect<N> {
        let bb = self.validate();
        Rect::new(bb.min.x, bb.min.y, bb.max.x - bb.min.x, bb.max.y - bb.min.y)
    }
}

impl<N> From<Rect<N>> for Aabr<N> where N: PrimaNum {
    fn from(rect: Rect<N>) -> Self {
        Self::new(
            Point2::new(rect.x, rect.y),
            Point2::new(rect.x + rect.w, rect.y + rect.h),
        )
    }
}