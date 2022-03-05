use std::{fmt::Display, ops::{Sub, Add}};

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

    /// Returns the distance between two coordinates.
    pub fn difference(self, other: Coord) -> Coord {
        let x = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let y = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        Coord { x, y }
    }

    /// Resturns the sum of the x and y coordinates.
    pub fn sum(&self) -> u32 {
        self.x + self.y
    }

    /// Returns the manhattan distance between two coordinates.
    pub fn manhattan_distance(self, other: Coord) -> u32 {
        self.difference(other).sum()
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

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference() {
        let a = Coord::new(10, 0);
        let b = Coord::new(10, 10);
        let diff = a.difference(b);
        assert_eq!(diff.x, 0);
        assert_eq!(diff.y, 10);
    }

    #[test]
    fn test_sum() {
        let a = Coord::new(10, 0);
        assert_eq!(a.sum(), 10);
    }

    #[test]
    fn test_manhattan_distance() {
        let a = Coord::new(10, 0);
        let b = Coord::new(10, 10);
        assert_eq!(a.manhattan_distance(b), 10);
    }
}