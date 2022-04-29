use crate::{Aabr, Interact, Line, PrimaFloat, Intersect, Shape, Collision};

impl<N> Interact<N, Line<N>> for Aabr<N> where N: PrimaFloat {
    fn collision(&self, line: &Line<N>) -> Option<crate::Collision<N>> {
        let n = line.closest_point(self.center());
        if !self.contains_point(&n) {
            return None;
        }

        let normal = line.normal();

        let x_overlap = (self.max.x - n.x).min(n.x - self.min.x);
        let y_overlap = (self.max.y - n.y).min(n.y - self.min.y);

        
        Some(Collision {
            penetration: x_overlap.min(y_overlap),
            normal,
            contact: n,
        })
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

#[cfg(test)]
mod tests {
    use crate::Point;

    use super::*;

    #[test]
    fn aabr_line_test() {
        let aabr = Aabr::new(Point::new(2.0, 2.0), Point::new(4.0, 4.0));
        let line = Line::<f32>::new(Point::new(2.5, 2.1), Point::new(2.5, 5.0));
        let collision = aabr.collision(&line).unwrap();
        println!("{:?}", collision);
    }
}