use crate::{
    Aabr, Circle, Collision, Interact, Intersect, PrimaFloat, Shape2, Vector,
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
        let n: Vector<N> = self.center().vector(aabr.center());
        let x_extent = aabr.half_extents().0;
        let y_extent = aabr.half_extents().1;

        let mut closest = n.clone();
        // Clamp point to edges of the AABB
        if closest.x < -x_extent {
            closest.x = -x_extent;
        } else if closest.x > x_extent {
            closest.x = x_extent;
        }
        if closest.y < -y_extent {
            closest.y = -y_extent;
        } else if closest.y > y_extent {
            closest.y = y_extent;
        }

        let mut inside = false;

        if n == closest {
            // Circle is inside the AABB, so we need to clamp the circle's center to the closest edge.
            inside = true;

            if n.x.abs() > n.y.abs() {
                // Clamp x
                closest.x = if n.x < N::zero() { -x_extent } else { x_extent };
            } else {
                // Clamp y
                closest.y = if n.y < N::zero() { -y_extent } else { y_extent };
            }
        }

        let normal: Vector<N> = n - closest;
        let d = normal.magnitude_squared();
        let r = self.radius;

        // The radius is shorter than distance to closest point and Circle not inside the AABB.
        if d > r * r && !inside {
            return None;
        }

        // Collect the sqrt of the distance
        let d = d.sqrt();

        if inside {
            // Collision normal needs to be flipped to point outside if circle was inside the AABB
            Some(Collision {
                penetration: r - d,
                normal: Vector::zero() - n,
            })
        } else {
            Some(Collision {
                penetration: r - d,
                normal: n,
            })
        }
    }

    fn nearest_extent(&self, _aabr: &Aabr<N>) -> crate::Point<N> {
        todo!()
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
