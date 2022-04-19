use crate::{Circle, PrimaFloat, Aabr, Shape2};

impl<N> From<Circle<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn from(c: Circle<N>) -> Self {
        c.bounding_box()
    }
}

impl<N> From<Aabr<N>> for Circle<N> where N: PrimaFloat {
    fn from(aabr: Aabr<N>) -> Self {
        let center = aabr.center();
        let radius = aabr.height().min(aabr.width()) / (N::one() + N::one());
        Self { center, radius }
    }
}