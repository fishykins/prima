use crate::{Interact, Aabr, Rect, Point2, PrimaNum, Collision};

impl<N> Interact<N, Rect<N>> for Aabr<N> where N: PrimaNum {
    fn collision(&self, other: &Rect<N>) -> Option<Collision<N>> {
        let other: Aabr<N> = Aabr::<N>::from(other.clone());
        self.collision(&other)
    }

    fn nearest_point(&self, _other: &Rect<N>) -> Option<Point2<N>> {
        todo!()
    }
}

impl<N> Interact<N, Aabr<N>> for Rect<N> where N: PrimaNum {
    fn collision(&self, other: &Aabr<N>) -> Option<Collision<N>> {
        Aabr::<N>::from(self.clone()).collision(other)
    }

    fn nearest_point(&self, _other: &Aabr<N>) -> Option<Point2<N>> {
        todo!()
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