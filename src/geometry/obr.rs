use crate::{Aabr, Angle, Extent, FlatShape, Mat2, Point, PrimaFloat, Shape, Interact, Collision};

/// Orientated bounding rectangle
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Obr<N>
where
    N: PrimaFloat,
{
    /// The center of the Obr.
    pub center: Point<N>,
    /// The half extents of the Obr.
    pub half_extents: Extent<N>,
    /// The rotation of the Obr.
    pub rotation: Angle<N>,
}

impl<N> Obr<N>
where
    N: PrimaFloat,
{
    /// Creates a new Obr from a center point, half extents, and rotation.
    pub fn new(center: Point<N>, half_extents: Extent<N>, rotation: Angle<N>) -> Self {
        Self {
            center,
            half_extents,
            rotation,
        }
    }

    /// Returns a localized [Aabr], centered around [0,0] and with no rotation applied.
    pub fn local_aabr(&self) -> Aabr<N> {
        Aabr::new(
            Point::new(-self.half_extents.w, -self.half_extents.h),
            Point::new(self.half_extents.w, self.half_extents.h),
        )
    }

    /// Returns a global [Aabr] of the Obr. 
    pub fn global_aabr(&self) -> Aabr<N> {
        Aabr::new(
            self.center - self.half_extents,
            self.center + self.half_extents,
        )
    }

    /// Returns the rotated [Obr] around the given point by the given angle.
    pub fn rotate_around_point(&self, point: Point<N>, angle: Angle<N>) -> Self {
        let rot_mat = angle.to_matrix();
        let v = self.center - point;
        let new_center = Point::new(v.x, v.y) * rot_mat + point;
        let new_rotation = self.rotation + angle;
        Self::new(new_center, self.half_extents, new_rotation)
    }
}

impl<N> Shape<N> for Obr<N>
where
    N: PrimaFloat,
{
    fn area(&self) -> N {
        self.half_extents.double().area()
    }

    fn circumference(&self) -> N {
        self.half_extents.sum() * N::from_u8(4).unwrap()
    }

    fn center(&self) -> Point<N> {
        self.center
    }

    fn bounding_box(&self) -> crate::Aabr<N> {
        let verts = self.vertices();
        let mut min_x = self.center.x;
        let mut min_y = self.center.y;
        let mut max_x = self.center.x;
        let mut max_y = self.center.y;

        for v in verts.iter() {
            min_x = min_x.min(v.x);
            min_y = min_y.min(v.y);
            max_x = max_x.max(v.x);
            max_y = max_y.max(v.y);
        }
        Aabr::new(Point::new(min_x, min_y), Point::new(max_x, max_y))
    }

    fn contains_point(&self, point: &Point<N>) -> bool {
        let rotation: Mat2<N> = self.rotation.into();
        let vector = *point - self.center;
        let rotated_point: Point<N> = (vector * rotation).into();
        let rotated_self = self.local_aabr();
        rotated_self.contains_point(&rotated_point)
    }

    fn nearest_point(&self, point: &Point<N>) -> Point<N> {
        let rotation = self.rotation.to_matrix();
        let vector = *point - self.center;
        let rotated_point: Point<N> = (vector * rotation).into();
        let rotated_self = self.local_aabr();
        let nearest_point = rotated_self.nearest_point(&rotated_point);
        let applied_nearest_point = nearest_point * self.rotation.mirror().to_matrix();
        self.center + applied_nearest_point
    }
}

impl<N> Interact<N> for Obr<N> where N: PrimaFloat {
    fn collision(&self, other: &Self) -> Option<Collision<N>> {
        let other_translated = other.rotate_around_point(self.center, -self.rotation);
        let self_axis_aligned = self.global_aabr();
        let other_edges = other_translated.edges();
        let rotation = self.rotation.to_matrix();

        for edge in other_edges.iter() {
            if let Some(mut collision) = self_axis_aligned.collision(edge) {
                collision.normal = collision.normal * rotation;
                collision.contact = collision.contact * rotation + self.center;
                return Some(collision);
            }
        }
        None
    }

    fn nearest_extent(&self, _other: &Self) -> Point<N> {
        todo!()
    }
}

impl<N> FlatShape<N> for Obr<N>
where
    N: PrimaFloat,
{
    fn vertices(&self) -> Vec<Point<N>> {
        let rotation_matrix: Mat2<N> = self.rotation.into();
        let a: Point<N> = Point::new(-self.half_extents.w, -self.half_extents.h);
        let b: Point<N> = Point::new(-self.half_extents.w, self.half_extents.h);
        let c: Point<N> = Point::new(self.half_extents.w, self.half_extents.h);
        let d: Point<N> = Point::new(self.half_extents.w, -self.half_extents.h);

        let a = a * rotation_matrix + self.center;
        let b = b * rotation_matrix + self.center;
        let c = c * rotation_matrix + self.center;
        let d = d * rotation_matrix + self.center;
        vec![a, b, c, d]
    }
}

impl<N> Into<Aabr<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn into(self) -> Aabr<N> {
        Aabr::new(
            self.center - self.half_extents,
            self.center + self.half_extents,
        )
    }
}

impl<N> From<Aabr<N>> for Obr<N>
where
    N: PrimaFloat,
{
    fn from(aabr: Aabr<N>) -> Self {
        let center = aabr.center();
        let half_extents = aabr.half_extents();
        let rotation = Angle::zero();
        Self::new(center, half_extents, rotation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    const IOTA: f32 = 0.01f32;

    #[test]
    fn rotate_test() {
        let rect: Obr<f32> = Obr::new(
            Point::new(1.0, 8.0),
            Extent::new(2.0, 1.0),
            Angle::from_degrees(45.0),
        );
        let verts = rect.vertices();
        let edges = rect.edges();

        assert_approx_eq!(verts[0].x, -1.12, IOTA);
        assert_approx_eq!(verts[0].y, 8.707, IOTA);
        assert_approx_eq!(edges[1].start.x, 0.29, IOTA);
        assert_approx_eq!(edges[1].start.y, 10.12, IOTA);
        assert_approx_eq!(edges[1].end.x, 3.12, IOTA);
        assert_approx_eq!(edges[1].end.y, 7.29, IOTA);

        let rect_2 = rect.rotate_around_point(Point::zero(), Angle::from_degrees(90.0));
        let verts = rect_2.vertices();
        assert_approx_eq!(verts[0].x, 8.707, IOTA);
        assert_approx_eq!(verts[0].y, 1.121, IOTA);
    }

    #[test]
    fn obr_point_test() {
        let rect = Obr::new(
            Point::splat(0.0),
            Extent::new(2.0, 1.0),
            Angle::from_degrees(25.0),
        );
        let point = Point::new(2.1, 0.0);
        assert!(rect.contains_point(&point));
        let point = Point::new(-1.5, 0.5);
        assert!(!rect.contains_point(&point));
    }

    #[test]
    fn ob_collision_test() {
        let a = Obr::new(
            Point::new(4.0, 3.5),
            Extent::new(2.0, 1.0),
            Angle::from_degrees(20.0),
        );

        let b = Obr::new(
            Point::new(7.0, 7.0),
            Extent::new(3.0, 2.0),
            Angle::from_degrees(-10.0),
        );

        let c = Obr::new(
            Point::new(6.9, 1.6),
            Extent::new(1.0, 1.0),
            Angle::from_degrees(60.0),
        );

        let collision = a.collision(&b);
        assert!(collision.is_none());

        let collision = a.collision(&c);
        assert!(collision.is_some());

    }
}
