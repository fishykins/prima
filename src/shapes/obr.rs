use crate::{core::{Point, Extent, Angle, Line, Collision}, nums::PrimaFloat, traits::{Distance, Nearest, Collide, Shape, Flat, LocalRotation, LocalPosition}};

use super::{Circle, Aabr};

/// An orientated bounding rectangle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Obr<N> {
    /// The center point of this rectangle.
    pub center: Point<N>,
    /// The extent of this rectangle.
    pub extent: Extent<N>,
    /// The rotation of this rectangle.
    pub rotation: Angle<N>,
}

impl<N> Shape<N> for Obr<N> where N: PrimaFloat {
    fn volume(&self) -> N {
        todo!()
    }

    fn circumference(&self) -> N {
        todo!()
    }

    fn bounding_rect(&self) -> Aabr<N> {
        todo!()
    }

    fn bounding_circle(&self) -> Circle<N> {
        todo!()
    }

    fn contains(&self, point: &Point<N>) -> bool {
        todo!()
    }
}

impl<N> Flat<N> for Obr<N> where N: PrimaFloat {
    fn vertices(&self) -> Vec<Point<N>> {
        todo!()
    }

    fn edges(&self) -> Vec<Line<N>> {
        todo!()
    }
}

impl<N> LocalPosition<N> for Obr<N> where N: PrimaFloat {
    fn position(&self) -> Point<N> {
        self.center
    }

    fn translate(&mut self, offset: &crate::core::Vector<N>) {
        todo!()
    }
}

impl<N> LocalRotation<N> for Obr<N> where N: PrimaFloat {
    fn rotate(&mut self, rotation: crate::core::Rotation<N>) {
        todo!()
    }

    fn rotation(&self) -> Angle<N> {
        todo!()
    }
}
//=================================================================//
//========================= POINT =================================//
//=================================================================//

impl<N> Distance<N, Point<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn squared_distance(&self, other: &Point<N>) -> N {
        self.nearest_point(other).squared_distance(other)
    }
}

impl<N> Nearest<N, Point<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, point: &Point<N>) -> Point<N> {
        todo!()
    }
}

//=================================================================//
//============================= LINE ==============================//
//=================================================================//

impl<N> Distance<N, Line<N>> for Obr<N> where N: PrimaFloat {
    fn squared_distance(&self, line: &Line<N>) -> N {
        todo!()
    }
}

impl<N> Nearest<N, Line<N>> for Obr<N> where N: PrimaFloat {
    fn nearest_point(&self, line: &Line<N>) -> Point<N> {
        todo!()
    }
}

impl<N> Collide<N, Line<N>> for Obr<N> where N: PrimaFloat {
    fn collision(&self, line: &Line<N>) -> Option<Collision<N>> {
        todo!()
    }

    fn enveloping(&self, line: &Line<N>) -> bool {
        todo!()
    }

    fn enveloped_by(&self, line: &Line<N>) -> bool {
        todo!()
    }
}

//=================================================================//
//============================ CIRCLE =============================//
//=================================================================//

impl<N> Distance<N, Circle<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn squared_distance(&self, circle: &Circle<N>) -> N {
        self.nearest_point(&circle.center).squared_distance(&circle.center)
    }
}

impl<N> Nearest<N, Circle<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, circle: &Circle<N>) -> Point<N> {
        self.nearest_point(&circle.center)
    }
}

impl<N> Collide<N, Circle<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn collision(&self, circle: &Circle<N>) -> Option<Collision<N>> {
        todo!()
    }

    fn intersecting(&self, circle: &Circle<N>) -> bool {
        todo!()
    }

    fn enveloping(&self, circle: &Circle<N>) -> bool {
        todo!()
    }

    fn enveloped_by(&self, circle: &Circle<N>) -> bool {
        self.vertices().iter().all(|v| circle.contains(v))
    }
}

//=================================================================//
//============================= AABR ==============================//
//=================================================================//

impl<N> Distance<N, Aabr<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn squared_distance(&self, aabr: &Aabr<N>) -> N {
        todo!()
    }
}

impl<N> Nearest<N, Aabr<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, aabr: &Aabr<N>) -> Point<N> {
        todo!()
    }
}

impl<N> Collide<N, Aabr<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn collision(&self, other: &Aabr<N>) -> Option<Collision<N>> {
        todo!()
    }

    fn enveloping(&self, other: &Aabr<N>) -> bool {
        todo!()
    }

    fn enveloped_by(&self, other: &Aabr<N>) -> bool {
        todo!()
    }
}

//=================================================================//
//============================== OBR ==============================//
//=================================================================//

impl<N> Distance<N, Obr<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn squared_distance(&self, obr: &Obr<N>) -> N {
        todo!()
    }
}

impl<N> Nearest<N, Obr<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, obr: &Obr<N>) -> Point<N> {
        todo!()
    }
}

impl<N> Collide<N, Obr<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn collision(&self, obr: &Obr<N>) -> Option<Collision<N>> {
        todo!()
    }

    fn enveloping(&self, obr: &Obr<N>) -> bool {
        todo!()
    }

    fn enveloped_by(&self, obr: &Obr<N>) -> bool {
        todo!()
    }
}
