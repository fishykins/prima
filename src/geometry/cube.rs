use crate::{PrimaNum, Point3};

/// A typical cube in 3D space.
pub struct Cube<N> {
    /// x coordinate of the cube's min.
    pub x: N,
    /// y coordinate of the cube's min.
    pub y: N,
    /// z coordinate of the cube's min.
    pub z: N,
    /// The cube's width.
    pub w: N,
    /// The cube's height.
    pub h: N,
    /// The cube's depth.
    pub d: N,
}


impl<N> Cube<N> where N: PrimaNum {
    /// Constructs a new rectangle.
    pub fn new(x: N, y: N, z: N, w: N, h: N, d: N) -> Self {
        Self { x, y, z, w, h, d }
    }

    /// Checks if the rectangle is valid (i.e. has a positive width and height) and is not equal to zero.
    /// 
    /// Due to the nature of a rectangle, being 'valid' is not something that can (or should) be enforced.
    /// To enact a validated rectangle, consider converting it into an Aabr.
    pub fn is_valid(&self) -> bool {
        self.w > N::zero() && self.h > N::zero() && self.d > N::zero()
    }

    /// Subdives the cube into 8 smaller cubes.
    pub fn subdivide(self) -> [Self; 8] {
        let two = N::one() + N::one();
        let w = self.w / two;
        let h = self.h / two;
        let d = self.d / two;
        [
            Self::new(self.x, self.y, self.z, w, h, d),
            Self::new(self.x + w, self.y, self.z, w, h, d),
            Self::new(self.x + w, self.y + h, self.z, w, h, d),
            Self::new(self.x, self.y + h, self.z, w, h, d),
            Self::new(self.x, self.y, self.z + d, w, h, d),
            Self::new(self.x + w, self.y, self.z + d, w, h, d),
            Self::new(self.x + w, self.y + h, self.z + d, w, h, d),
            Self::new(self.x, self.y + h, self.z + d, w, h, d),
        ]
    }

    /// Returns the minimum point of the [Rect]. 
    pub fn min(&self) -> Point3<N> {
        Point3::new(self.x, self.y, self.z)
    }

    /// Returns the maximum point of the [Rect].
    pub fn max(&self) -> Point3<N> {
        Point3::new(self.x + self.w, self.y + self.h, self.z + self.d)
    }
}
