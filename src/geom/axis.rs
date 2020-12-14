#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Axis {
    Vertical,
    Horizontal,
    None,
}

impl Axis {
    pub fn perpendicular(self) -> Self {
        match self {
            Axis::Vertical => Axis::Horizontal,
            Axis::Horizontal => Axis::Vertical,
            _ => Axis::None,
        }
    }
}