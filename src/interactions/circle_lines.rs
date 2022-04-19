use crate::{Circle, Point2, Intersect, Line2, PrimaFloat, Shape2, FastDistance, Collide};

impl<N> Collide<Line2<N>> for Circle<N> where N: PrimaFloat {
    type Output = Point2<N>;

    /// Returns the closest point on the line to the circle.
    fn collision(&self, line: &Line2<N>) -> Option<Self::Output> {
        let start_inside = self.contains_point(&line.start);
        let end_inside = self.contains_point(&line.end);
        if start_inside || end_inside {
            return None;
        }

        let length = line.length();
        let dot = (((self.center.x - line.start.x) * (line.end.x - line.start.x))
            + ((self.center.y - line.start.y) * (line.end.y - line.start.y)))
            / length.powi(2);

        let closest_x = line.start.x + dot * (line.end.x - line.start.x);
        let closest_y = line.start.y + dot * (line.end.y - line.start.y);
        let closest = Point2::new(closest_x, closest_y);
        if !line.contains_point(closest) {
            return None;
        }
        if self.center.distance_squared(&closest) <= self.radius * self.radius {
            return Some(closest);
        }
        None
    }
}

impl<N> Intersect<Line2<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn intersecting(&self, line: &Line2<N>) -> bool {
        self.colliding(line)
    }
}
