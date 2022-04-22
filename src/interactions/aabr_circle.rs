use crate::{
    Aabr, Circle, Collision, Interact, Intersect, PrimaFloat, Shape, Distance,
};

impl<N> From<Circle<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn from(c: Circle<N>) -> Self {
        c.bounding_box()
    }
}

impl<N> From<Aabr<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn from(aabr: Aabr<N>) -> Self {
        let center = aabr.center();
        let radius = aabr.height().min(aabr.width()) / (N::one() + N::one());
        Self { center, radius }
    }
}

impl<N> Interact<N, Aabr<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn collision(&self, aabr: &Aabr<N>) -> Option<crate::Collision<N>> {
        let n = self.nearest_extent(aabr);
        if !self.contains_point(&n) {
            return None;
        }
        let penetration = self.radius - n.distance(&self.center);
        let normal = n - self.center;

        // TODO: Resolve when circle center is within the aabr. The center of the circle needs to be clipped to the closest edge of the Aabr, and the normal needs to be flipped.

        Some(Collision {
            penetration,
            normal,
        })
    }

    fn nearest_extent(&self, aabr: &Aabr<N>) -> crate::Point<N> {
        let mut p = self.center();
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

impl<N> Intersect<Circle<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn intersecting(&self, circle: &Circle<N>) -> bool {
        if self.contains_point(&circle.center) {
            return true;
        }
        let aabr_center = self.center();
        let two = N::one() + N::one();
        let half_width = self.width() / two;
        let half_height = self.height() / two;

        let circle_distance_x = (circle.center.x - aabr_center.x).abs();
        let circle_distance_y = (circle.center.y - aabr_center.y).abs();

        if circle_distance_x > half_width + circle.radius {
            return false;
        }
        if circle_distance_y > half_height + circle.radius {
            return false;
        }

        if circle_distance_x <= half_width {
            return true;
        }
        if circle_distance_y <= half_height {
            return true;
        }

        let corner_dist_sq =
            (circle_distance_x - half_width).powi(2) + (circle_distance_y - half_height).powi(2);

        corner_dist_sq <= circle.radius * circle.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Circle, Aabr, Point, Vector};

    #[test]
    fn aabr_circle_test() {
        let circle = Circle::new(Point::new(32.0, 32.0), 8.0);
        let square = Aabr::new(Point::new(0.0, 0.0), Point::new(30.0, 30.0));
        let nearest_point_on_square = circle.nearest_extent(&square);
        assert_eq!(nearest_point_on_square, Point::new(30.0, 30.0));
        let collision = circle.collision(&square).unwrap();
        assert_eq!(collision.penetration, 8.0 - 2. * 2.0f32.sqrt());
        assert_eq!(collision.normal, Vector::new(2.0, 2.0));
    }
}