
/// Vertical, Horizontal and none!
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Axis {
    Vertical,
    Horizontal,
    None,
    Both,
}

impl Axis {
    pub fn perpendicular(self) -> Self {
        match self {
            Axis::Vertical => Axis::Horizontal,
            Axis::Horizontal => Axis::Vertical,
            Axis::Both => Axis::Both,
            _ => Axis::None,
        }
    }
}