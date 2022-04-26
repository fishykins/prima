use crate::{PrimaNum, Aabr, Point};

/// An extent used to describe the length of an axis pair.
pub struct Extent<N> {
    /// Width
    pub w: N,
    /// Height
    pub h: N,
}

impl<N> Extent<N> where N: PrimaNum {
    /// Builds a new extent from two values.
    pub fn new(w: N, h: N) -> Self {
        Self { w, h }
    }

    /// Returns the half extent for width.
    pub fn half_width(&self) -> N {
        self.w / Self::two()
    }

    /// Returns the half extent for height.
    pub fn half_height(&self) -> N {
        self.h / Self::two()
    }

    /// Returns the half extent for both width and height.
    pub fn half_extents(self) -> Self {
        Self::new(self.w / Self::two(), self.h / Self::two())
    }

    /// Doubles the extent (to get from half extents to full extents).
    pub fn from_half_extents(half_extents: Self) -> Self {
        Self::new(half_extents.w * Self::two(), half_extents.h * Self::two())
    }

    fn two() -> N {
        N::one() + N::one()
    }

    /// Creates an Aabr from this extent, combined with a given center point.
    pub fn to_rect(self, center: Point<N>) -> Aabr<N> {
        let (w, h) = self.half_extents().into();
        let min = center - Point::new(w, h);
        let max = center + Point::new(w, h);
        Aabr::new(min.into(), max.into())
    }
}

impl<N> Into<Aabr<N>> for Extent<N> where N: PrimaNum {
    fn into(self) -> Aabr<N> {
        Aabr::new(
            Point::new(N::zero(), N::zero()),
            Point::new(self.w, self.h),
        )
    }
}

impl<N> From<Aabr<N>> for Extent<N> where N: PrimaNum {
    fn from(aabr: Aabr<N>) -> Self {
        aabr.extents()
    }
}

impl<N> From<(N, N)> for Extent<N> where N: PrimaNum {
    fn from(t: (N, N)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl<N> Into<(N, N)> for Extent<N> where N: PrimaNum {
    fn into(self) -> (N, N) {
        (self.w, self.h)
    }
}