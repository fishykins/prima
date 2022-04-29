use crate::{Interact, Obr, PrimaFloat, Circle, Rotation, Shape, Intersect};

impl<N> Interact<N, Obr<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn collision(&self, other: &Obr<N>) -> Option<crate::Collision<N>> {
        // rotate the circle around the obr until the obr is axis aligned.
        let aabr = other.global_aabr();
        let rotation: Rotation<N> = other.rotation.into();
        let circle = self.rotate_around_point(aabr.center(), -rotation);
        circle.collision(&aabr)
    }

    fn nearest_extent(&self, other: &Obr<N>) -> crate::Point<N> {
        let aabr = other.global_aabr();
        let rotation: Rotation<N> = other.rotation.into();
        let circle = self.rotate_around_point(aabr.center(), -rotation);
        let nearest = circle.nearest_extent(&aabr);
        nearest.rotate_around(aabr.center(), rotation)
    }
}

impl<N> Intersect<Obr<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn intersecting(&self, obr: &Obr<N>) -> bool {
        let aabr = obr.global_aabr();
        let rotation: Rotation<N> = obr.rotation.into();
        let circle = self.rotate_around_point(aabr.center(), -rotation);
        aabr.intersecting(&circle)
    }
}
