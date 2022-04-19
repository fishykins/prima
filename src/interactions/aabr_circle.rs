use crate::{Aabr, Circle, Intersect, PrimaFloat, Shape2};

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
