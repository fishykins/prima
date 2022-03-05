use super::Axis;

/// Defines the six planes of linear movement available in 3D space. 
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Transverse3D {
    /// Upward direction
    Up,
    /// Downward direction
    Down,
    /// Leftward direction
    Left,
    /// Rightward direction
    Right,
    /// Forward direction
    Forward,
    /// Backward direction
    Backward,
}

impl Transverse3D {
    /// Returns the opposite Transverse of self.
    pub fn opposite(self) -> Self {
        match self {
            Transverse3D::Up => Transverse3D::Down,
            Transverse3D::Down => Transverse3D::Up,
            Transverse3D::Left => Transverse3D::Right,
            Transverse3D::Right => Transverse3D::Left,
            Transverse3D::Forward => Transverse3D::Backward,
            Transverse3D::Backward => Transverse3D::Forward,
        }
    }

    /// Converts Transverse into ['Axis'].
    pub fn axis(self) -> Axis {
        match self {
            Transverse3D::Up => Axis::Vertical,
            Transverse3D::Down => Axis::Vertical,
            Transverse3D::Left => Axis::Horizontal,
            Transverse3D::Right => Axis::Horizontal,
            Transverse3D::Forward => Axis::Vertical,
            Transverse3D::Backward => Axis::Vertical,
        }
    }
}

/// Defines the four planes of linear movement available in 2D space. 
pub enum Transverse2D {
    /// Upward direction
    Up,
    /// Downward direction
    Down,
    /// Leftward direction
    Left,
    /// Rightward direction
    Right,
}