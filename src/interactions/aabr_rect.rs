use crate::{Aabr, Rect, Point2, PrimaNum};

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