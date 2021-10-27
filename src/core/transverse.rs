use super::Axis;

/// Defines the six planes of linear movement avalible in 3D space. 
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Transverse {
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

impl Transverse {
    /// Returns the opposite Transverse of self.
    pub fn opposite(self) -> Self {
        match self {
            Transverse::Up => Transverse::Down,
            Transverse::Down => Transverse::Up,
            Transverse::Left => Transverse::Right,
            Transverse::Right => Transverse::Left,
            Transverse::Forward => Transverse::Backward,
            Transverse::Backward => Transverse::Forward,
        }
    }

    /// Converts Transverse into ['Axis'].
    pub fn axis(self) -> Axis {
        match self {
            Transverse::Up => Axis::Vertical,
            Transverse::Down => Axis::Vertical,
            Transverse::Left => Axis::Horizontal,
            Transverse::Right => Axis::Horizontal,
            Transverse::Forward => Axis::Vertical,
            Transverse::Backward => Axis::Vertical,
        }
    }
}