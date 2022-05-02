use crate::{
    core::{Angle, Collision, Point, Rotation, Vector},
    nums::PrimaFloat,
    traits::{Collide, Curved, Distance, LocalPosition, LocalRotation, Nearest, Shape},
};

use super::Aabr;

/// A circle. It is big and round and has a radius.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Circle<N> {
    /// The radius of the circle.
    pub radius: N,
    /// The center of the circle.
    pub center: Point<N>,
}

impl<N> Circle<N> {
    /// Creates a new circle from a center point and radius.
    pub fn new(center: Point<N>, radius: N) -> Self {
        Self { center, radius }
    }
}

impl<N> Shape<N> for Circle<N>
where
    N: PrimaFloat,
{
    fn volume(&self) -> N {
        let pi = N::pi();
        pi * self.radius * self.radius
    }

    fn circumference(&self) -> N {
        let pi = N::pi();
        (pi + pi) * self.radius
    }

    fn bounding_rect(&self) -> Aabr<N> {
        let v = self.radius + self.radius;
        Aabr::from_point(self.center, v, v)
    }

    fn bounding_circle(&self) -> Circle<N> {
        self.clone()
    }

    fn contains(&self, point: &Point<N>) -> bool {
        let d = self.center.distance(point);
        d <= self.radius
    }
}

impl<N> LocalPosition<N> for Circle<N>
where
    N: PrimaFloat,
{
    fn position(&self) -> Point<N> {
        self.center
    }

    fn translate(&mut self, offset: &crate::core::Vector<N>) {
        self.center += *offset;
    }
}

impl<N> LocalRotation<N> for Circle<N>
where
    N: PrimaFloat,
{
    fn rotate(&mut self, _: Rotation<N>) {}

    fn rotation(&self) -> Angle<N> {
        Angle::zero()
    }
}

impl<N> Curved<N> for Circle<N> where N: PrimaFloat {}

impl<N> Distance<N, Point<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn squared_distance(&self, other: &Point<N>) -> N {
        self.nearest_point(other).squared_distance(other)
    }
}

impl<N> Distance<N, Circle<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn squared_distance(&self, other: &Circle<N>) -> N {
        let a = self.nearest_point(other);
        let b = other.nearest_point(self);
        a.squared_distance(&b)
    }
}

impl<N> Nearest<N, Point<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, point: &Point<N>) -> Point<N> {
        let v = *point - self.center;
        self.center + v.normalize() * self.radius
    }
}

impl<N> Nearest<N, Circle<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, other: &Circle<N>) -> Point<N> {
        let p = other.center;
        self.nearest_point(&p)
    }
}

impl<N> Collide<N> for Circle<N>
where
    N: PrimaFloat,
{
    fn collision(&self, other: &Self) -> Option<crate::core::Collision<N>> {
        let d = self.center.distance(&other.center);
        let r = self.radius + other.radius;
        if d <= r {
            let normal: Vector<N> = (other.center - self.center).into();
            if normal == Vector::zero() {
                Some(Collision::new(
                    self.center,
                    Vector::new(N::one(), N::zero()),
                    self.radius,
                ))
            } else {
                Some(Collision::new(
                    self.center + normal.normalize() * (self.radius - r - d),
                    normal,
                    r - d,
                ))
            }
        } else {
            None
        }
    }

    fn intersecting(&self, other: &Self) -> bool {
        self.center.squared_distance(&other.center)
            <= (self.radius + other.radius) * (self.radius + other.radius)
    }

    fn enveloping(&self, other: &Self) -> bool {
        let d = self.center.distance(&other.center);
        d + other.radius <= self.radius
    }

    fn enveloped_by(&self, other: &Self) -> bool {
        let d = self.center.distance(&other.center);
        d + self.radius <= other.radius
    }
}
