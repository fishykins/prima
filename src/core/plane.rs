
/// Describes a plane in 3D space.
#[derive(Debug, Clone, Copy)]
pub enum Plane3D {
    /// The X plane.
    X,
    /// The Y plane.
    Y,
    /// The Z plane.
    Z,
}


/// Describes a plane in 2D space.
#[derive(Debug, Clone, Copy)]
pub enum Plane2D {
    /// The X plane.
    X,
    /// The Y plane.
    Y
}

impl From<Plane3D> for Plane2D {
    fn from(plane: Plane3D) -> Self {
        match plane {
            Plane3D::X => Plane2D::X,
            Plane3D::Y => Plane2D::Y,
            Plane3D::Z => Plane2D::X,
        }
    }
}

impl From<Plane2D> for Plane3D {
    fn from(plane: Plane2D) -> Self {
        match plane {
            Plane2D::X => Plane3D::X,
            Plane2D::Y => Plane3D::Y,
        }
    }
}