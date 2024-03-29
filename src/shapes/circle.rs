use crate::{
    core::{Collision, Line, Point, Rotation, Vector},
    nums::PrimaFloat,
    traits::{Collide, Curved, Distance, LocalPosition, LocalRotation, Magnitude, Nearest, Shape, Flat},
};

use super::{Aabr, Obr};

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

/// While a circle is not rotatable, it is possible to rotate it around a point.
/// Implimenting this trait gives an auto-implimentation of the `rotate_around` method.
impl<N> LocalRotation<N> for Circle<N>
where
    N: PrimaFloat,
{
    fn rotate(&mut self, _: Rotation<N>) {}

    fn rotation(&self) -> Rotation<N> {
        Rotation::zero()
    }
}

impl<N> Curved<N> for Circle<N> where N: PrimaFloat {}

//=================================================================//
//========================= POINT =================================//
//=================================================================//

impl<N> Distance<N, Point<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn distance_squared(&self, point: &Point<N>) -> N {
        self.nearest_point(point).distance_squared(point)
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

//=================================================================//
//============================= LINE ==============================//
//=================================================================//

impl<N> Nearest<N, Line<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, line: &Line<N>) -> Point<N> {
        let length = line.magnitude();
        let dot = (((self.center.x - line.start.x) * (line.end.x - line.start.x))
            + ((self.center.y - line.start.y) * (line.end.y - line.start.y)))
            / length.powi(2);

        let closest_x = line.start.x + dot * (line.end.x - line.start.x);
        let closest_y = line.start.y + dot * (line.end.y - line.start.y);
        Point::new(closest_x, closest_y)
    }
}

impl<N> Distance<N, Line<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn distance_squared(&self, line: &Line<N>) -> N {
        let p = self.nearest_point(line);
        p.distance_squared(&line.start)
    }
}

impl<N> Collide<N, Line<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn collision(&self, line: &Line<N>) -> Option<Collision<N>> {
        let start_inside = self.contains(&line.start);
        let end_inside = self.contains(&line.end);
        if start_inside || end_inside {
            return None;
        }

        let closest = self.nearest_point(line);
        if !closest.on_line(line) {
            return None;
        }

        let dist_squared = self.center.distance_squared(&closest);

        if dist_squared <= self.radius * self.radius {
            return Some(Collision::new(
                closest,
                (closest - self.center).normalize(),
                self.radius - dist_squared.sqrt(),
            ));
        }
        None
    }

    fn intersecting(&self, line: &Line<N>) -> bool {
        let start_inside = self.contains(&line.start);
        let end_inside = self.contains(&line.end);
        if start_inside || end_inside {
            return true;
        }
        let closest = self.nearest_point(line);
        self.contains(&closest)
    }

    fn enveloping(&self, line: &Line<N>) -> bool {
        self.contains(&line.start) && self.contains(&line.end)
    }

    fn enveloped_by(&self, _: &Line<N>) -> bool {
        false
    }
}

//=================================================================//
//============================ CIRCLE =============================//
//=================================================================//

impl<N> Distance<N, Circle<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn distance_squared(&self, other: &Circle<N>) -> N {
        let ar = self.radius.powi(2);
        let br = other.radius.powi(2);
        let total_dist_sqrd = self.center.distance_squared(&other.center);
        total_dist_sqrd - ar - br
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
        self.center.distance_squared(&other.center)
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

//=================================================================//
//============================= AABR ==============================//
//=================================================================//

impl<N> Distance<N, Aabr<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn distance_squared(&self, aabr: &Aabr<N>) -> N {
        let a = self.nearest_point(aabr);
        let b = aabr.nearest_point(self);
        a.distance_squared(&b)
    }
}

impl<N> Nearest<N, Aabr<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, aabr: &Aabr<N>) -> Point<N> {
        let mut p = self.center;
        if p.x > aabr.max.x {
            p.x = aabr.max.x;
        } else if p.x < aabr.min.x {
            p.x = aabr.min.x;
        }
        if p.y > aabr.max.y {
            p.y = aabr.max.y;
        } else if p.y < aabr.min.y {
            p.y = aabr.min.y;
        }
        p
    }
}

impl<N> Collide<N, Aabr<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn collision(&self, aabr: &Aabr<N>) -> Option<Collision<N>> {
        aabr.collision(self)
    }

    fn intersecting(&self, aabr: &Aabr<N>) -> bool {
        aabr.intersecting(self)
    }

    fn enveloping(&self, aabr: &Aabr<N>) -> bool {
        aabr.enveloped_by(self)
    }

    fn enveloped_by(&self, aabr: &Aabr<N>) -> bool {
        aabr.enveloping(self)
    }
}

//=================================================================//
//============================== OBR ==============================//
//=================================================================//

impl<N> Distance<N, Obr<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn distance_squared(&self, obr: &Obr<N>) -> N {
        let a = self.nearest_point(obr);
        let b = obr.nearest_point(self);
        a.distance_squared(&b)
    }
}

impl<N> Nearest<N, Obr<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, obr: &Obr<N>) -> Point<N> {
        let rotation = obr.rotation.into();
        let aabr = obr.as_aabr();
        let mut circle = self.clone();
        circle.rotate_around(obr.center, rotation);
        let p = circle.nearest_point(&aabr);
        p.rotate_around(obr.center, -rotation)
    }
}

impl<N> Collide<N, Obr<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn collision(&self, obr: &Obr<N>) -> Option<Collision<N>> {
        let rotation = obr.rotation.into();
        let aabr = obr.as_aabr();
        let mut circle = self.clone();
        circle.rotate_around(obr.center, rotation);

        let collision = circle.collision(&aabr);
        if let Some(collision) = collision {
            let normal = collision.normal * -rotation;
            let point = collision.point.rotate_around(obr.center, -rotation);
            Some(Collision::new(point, normal, collision.depth))
        } else {
            None
        }
    }

    fn enveloping(&self, obr: &Obr<N>) -> bool {
        obr.vertices().iter().all(|v| self.contains(v))
    }

    fn enveloped_by(&self, obr: &Obr<N>) -> bool {
        let rotation = obr.rotation.into();
        let aabr = obr.as_aabr();
        let mut circle = self.clone();
        circle.rotate_around(obr.center, rotation);
        aabr.enveloping(&circle)
    }
}
