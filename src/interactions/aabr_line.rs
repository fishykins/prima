use crate::{Aabr, Interact, Line, PrimaFloat, Intersect, Shape, Vector, Collision};

impl<N> Interact<N, Line<N>> for Aabr<N> where N: PrimaFloat {
    fn collision(&self, line: &Line<N>) -> Option<crate::Collision<N>> {
        let n = line.closest_point(self.center());
        if !self.contains_point(&n) {
            return None;
        }

        let x_overlap = (self.max.x - n.x).max(n.x - self.min.x);
        let y_overlap = (self.max.y - n.y).max(n.y - self.min.y);

        if x_overlap < y_overlap {
            let normal = if (self.max.x - n.x) > (n.x - self.min.x) {
                Vector::new(-N::one(), N::zero())
            } else {
                Vector::new(N::one(), N::zero())
            };
            return Some(Collision {
                penetration: x_overlap,
                normal,
            });
        } else {
            let normal = if (self.max.y - n.y) > (n.y - self.min.y) {
                Vector::new(N::zero(), -N::one())
            } else {
                Vector::new(N::zero(), N::one())
            };
            return Some(Collision {
                penetration: y_overlap,
                normal,
            });
        }
    }

    fn nearest_extent(&self, _line: &Line<N>) -> crate::Point<N> {
        todo!()
    }
}

impl<N> Intersect<Line<N>> for Aabr<N> where N: PrimaFloat {
    fn intersecting(&self, line: &Line<N>) -> bool {
        let n = line.closest_point(self.center());
        self.contains_point(&n)
    }
}