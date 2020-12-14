use super::Axis;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub enum Transverse {
    Up,
    Down,
    Left,
    Right,
    Forward,
    Backward,
}

impl Transverse {
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