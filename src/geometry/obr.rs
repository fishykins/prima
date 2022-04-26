use crate::{Point, Extent, Angle, PrimaFloat, Shape, Aabr, FlatShape, Mat2};

/// Orientated bounding rectangle
pub struct Obr<N> where N: PrimaFloat {
    /// The center of the Obr.
    pub center: Point<N>,
    /// The half extents of the Obr.
    pub half_extents: Extent<N>,
    /// The rotation of the Obr.
    pub rotation: Angle<N>,
}

impl<N> Obr<N> where N: PrimaFloat {
    /// Creates a new Obr from a center point, half extents, and rotation. 
    pub fn new(center: Point<N>, half_extents: Extent<N>, rotation: Angle<N>) -> Self {
        Self {
            center,
            half_extents,
            rotation,
        }
    }
}

impl<N> Shape<N> for Obr<N> where N: PrimaFloat {
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
        todo!()
    }

    fn contains_point(&self, _point: &Point<N>) -> bool {
        todo!()
    }

    fn nearest_point(&self, _point: &Point<N>) -> Point<N> {
        todo!()
    }
}

impl<N> FlatShape<N> for Obr<N> where N: PrimaFloat {
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

impl<N> Into<Aabr<N>> for Obr<N> where N: PrimaFloat {
    fn into(self) -> Aabr<N> {
        Aabr::new(self.center - self.half_extents, self.center + self.half_extents)
    }
}

impl<N> From<Aabr<N>> for Obr<N> where N: PrimaFloat {
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

    #[test]
    fn rotate_test() {
        let _rect = Obr::new(Point::splat(10.0), Extent::new(2.0, 2.0), Angle::from_degrees(45.0));
        
    }
}