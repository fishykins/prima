use std::fmt::Display;

/// An intiger based, unsigned 2D coordinate.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coord {
    /// X coordinate.
    pub x: u32,
    /// Y coordinate.
    pub y: u32,
}

impl Coord {
    /// Builds a new coordinate.
    pub fn new(x: u32, y: u32) -> Coord {
        Coord { x, y }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coord({},{})", self.x, self.y)
    }
}

impl Into<(u32, u32)> for Coord {
    fn into(self) -> (u32, u32) {
        (self.x, self.y)
    }
}

impl From<(u32, u32)> for Coord {
    fn from(t: (u32, u32)) -> Coord {
        Coord { x: t.0, y: t.1 }
    }
}
