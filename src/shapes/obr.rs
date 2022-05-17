use crate::{prelude::*, core::project_shape_to_axis_pair};

use super::{Aabr, Circle};

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

impl<N> Obr<N>
where
    N: PrimaFloat,
{
    /// Creates a new orientated bounding rectangle.
    pub fn new(center: Point<N>, extent: Extent<N>, rotation: Angle<N>) -> Self {
        Self {
            center,
            extent,
            rotation,
        }
    }

    /// Converts this rectangle into a bounding rectangle at the same position, essentially discarding rotation.
    pub fn as_aabr(&self) -> Aabr<N> {
        Aabr::from_point(self.center, self.extent.width(), self.extent.height())
    }

    /// Converts this rectangle into a circle at the origin, without any rotation.
    pub fn as_local_aabr(&self) -> Aabr<N> {
        Aabr::from_point(Point::zero(), self.extent.width(), self.extent.height())
    }

    /// Returns the normal of the x axis in global space.
    pub fn x_axis(&self) -> Vector<N> {
        let x: Vector<N> = Vector::right();
        let r: Rotation<N> = self.rotation.into();
        x * r
    }

    /// Returns the normal of the y axis in global space.
    pub fn y_axis(&self) -> Vector<N> {
        let y: Vector<N> = Vector::up();
        let r: Rotation<N> = self.rotation.into();
        y * r
    }
}

impl<N> Shape<N> for Obr<N>
where
    N: PrimaFloat,
{
    fn volume(&self) -> N {
        self.extent.volume()
    }

    fn circumference(&self) -> N {
        self.extent.double().sum()
    }

    fn bounding_rect(&self) -> Aabr<N> {
        let mut min = Point::new(N::infinity(), N::infinity());
        let mut max = Point::new(N::neg_infinity(), N::neg_infinity());

        for v in self.vertices() {
            if v.x < min.x {
                min.x = v.x;
            } else if v.x > max.x {
                max.x = v.x;
            }
            if v.y < min.y {
                min.y = v.y;
            } else if v.y > max.y {
                max.y = v.y;
            }
        }
        Aabr::new(min, max)
    }

    fn bounding_circle(&self) -> Circle<N> {
        let center = self.position();
        let corner = self.center + self.extent.half();
        let r = self.center.distance(&corner);
        Circle::new(center, r)
    }

    fn contains(&self, point: &Point<N>) -> bool {
        let aabr = Aabr::new(
            Point::zero(),
            Point::new(self.extent.width(), self.extent.height()),
        );
        let point_relative = Point::new(
            point.x - self.center.x - self.extent.half_width(),
            point.y - self.center.y - self.extent.half_height(),
        ) * -self.rotation();
        aabr.contains(&point_relative)
    }
}

impl<N> Flat<N> for Obr<N>
where
    N: PrimaFloat,
{
    fn vertices(&self) -> Vec<Point<N>> {
        let e = self.extent;
        let r = self.rotation().to_matrix();
        let a = self.center - e;
        let b = Point::new(
            self.center.x - e.half_width(),
            self.center.y + e.half_height(),
        );
        let c = self.center + e;
        let d = Point::new(
            self.center.x + e.half_width(),
            self.center.y - e.half_height(),
        );
        vec![
            a.rotate_around_mat(self.center, r),
            b.rotate_around_mat(self.center, r),
            c.rotate_around_mat(self.center, r),
            d.rotate_around_mat(self.center, r),
        ]
    }
}

impl<N> LocalPosition<N> for Obr<N>
where
    N: PrimaFloat,
{
    fn position(&self) -> Point<N> {
        self.center
    }

    fn translate(&mut self, offset: &Vector<N>) {
        self.center = self.center + *offset;
    }
}

impl<N> LocalRotation<N> for Obr<N>
where
    N: PrimaFloat,
{
    fn rotate(&mut self, rotation: Rotation<N>) {
        self.rotation += rotation;
    }

    fn rotation(&self) -> Rotation<N> {
        Rotation::from_radians(self.rotation.as_radians())
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
    fn nearest_point(&self, _point: &Point<N>) -> Point<N> {
        todo!()
    }
}

//=================================================================//
//============================= LINE ==============================//
//=================================================================//

impl<N> Distance<N, Line<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn squared_distance(&self, _line: &Line<N>) -> N {
        todo!()
    }
}

impl<N> Nearest<N, Line<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, _line: &Line<N>) -> Point<N> {
        todo!()
    }
}

impl<N> Collide<N, Line<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn collision(&self, _line: &Line<N>) -> Option<Collision<N>> {
        todo!()
    }

    fn enveloping(&self, _line: &Line<N>) -> bool {
        todo!()
    }

    fn enveloped_by(&self, _line: &Line<N>) -> bool {
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
        self.nearest_point(&circle.center)
            .squared_distance(&circle.center)
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
    fn collision(&self, _circle: &Circle<N>) -> Option<Collision<N>> {
        todo!()
    }

    fn intersecting(&self, _circle: &Circle<N>) -> bool {
        todo!()
    }

    fn enveloping(&self, _circle: &Circle<N>) -> bool {
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
        aabr.squared_distance(self)
    }
}

impl<N> Nearest<N, Aabr<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, _aabr: &Aabr<N>) -> Point<N> {
        todo!()
    }
}

impl<N> Collide<N, Aabr<N>> for Obr<N>
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

impl<N> Distance<N, Obr<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn squared_distance(&self, _obr: &Obr<N>) -> N {
        todo!()
    }
}

impl<N> Nearest<N, Obr<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, _obr: &Obr<N>) -> Point<N> {
        todo!()
    }
}

impl<N> Collide<N, Obr<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn collision(&self, _obr: &Obr<N>) -> Option<Collision<N>> {
        todo!()
    }

    fn intersecting(&self, other: &Obr<N>) -> bool {
        let x_axis = self.x_axis();
        let y_axis = self.y_axis();
        let (a_x, a_y) = project_shape_to_axis_pair(self, x_axis, y_axis);
        let (b_x, b_y) = project_shape_to_axis_pair(other, x_axis, y_axis);

        // Compare!
        if !a_x.intersecting(&b_x) || !a_y.intersecting(&b_y) {
            return false;
        }

        let x_axis = other.x_axis();
        let y_axis = other.y_axis();

        let (a_x, a_y) = project_shape_to_axis_pair(self, x_axis, y_axis);
        let (b_x, b_y) = project_shape_to_axis_pair(other, x_axis, y_axis);

        a_x.intersecting(&b_x) && a_y.intersecting(&b_y)
    }

    fn enveloping(&self, _obr: &Obr<N>) -> bool {
        todo!()
    }

    fn enveloped_by(&self, _obr: &Obr<N>) -> bool {
        todo!()
    }
}
