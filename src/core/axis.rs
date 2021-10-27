
/// Helper enum to define vertical and horizontal axis. Can be 'None' and 'Both'.
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Axis {
    /// Up, down
    Vertical,
    /// Left, Right
    Horizontal,
    /// No axis, sad face.
    None,
    /// ALL THE AXIS
    Both,
}

impl Axis {
    /// Returns the perpendicular axis. 
    pub fn perpendicular(self) -> Self {
        match self {
            Axis::Vertical => Axis::Horizontal,
            Axis::Horizontal => Axis::Vertical,
            Axis::Both => Axis::Both,
            _ => Axis::None,
        }
    }
}