use crate::{Circle, Collision, FastDistance, Interact, Intersect, Line, Point, PrimaFloat, Shape};

impl<N> Interact<N, Line<N>> for Circle<N>
where
    N: PrimaFloat,
{
    /// Returns the closest point on the line to the circle.
    fn collision(&self, line: &Line<N>) -> Option<Collision<N>> {
        let start_inside = self.contains_point(&line.start);
        let end_inside = self.contains_point(&line.end);
        if start_inside || end_inside {
            return None;
        }

        let closest = self.nearest_extent(line);
        if !line.contains_point(closest) {
            return None;
        }

        let dist_squared = self.center.distance_squared(&closest);

        if dist_squared <= self.radius * self.radius {
            return Some(Collision::<N> {
                penetration: self.radius - dist_squared.sqrt(),
                normal: closest.vector(self.center).normalize(),
                contact: closest,
            });
        }
        None
    }

    fn nearest_extent(&self, line: &Line<N>) -> Point<N> {
        let length = line.length();
        let dot = (((self.center.x - line.start.x) * (line.end.x - line.start.x))
            + ((self.center.y - line.start.y) * (line.end.y - line.start.y)))
            / length.powi(2);

        let closest_x = line.start.x + dot * (line.end.x - line.start.x);
        let closest_y = line.start.y + dot * (line.end.y - line.start.y);
        Point::new(closest_x, closest_y)
    }
}

impl<N> Intersect<Line<N>> for Circle<N>
where
    N: PrimaFloat,
{
    fn intersecting(&self, line: &Line<N>) -> bool {
        let start_inside = self.contains_point(&line.start);
        let end_inside = self.contains_point(&line.end);
        if start_inside || end_inside {
            return true;
        }
        let closest = self.nearest_extent(line);
        self.contains_point(&closest)
    }
}
