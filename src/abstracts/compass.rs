use serde::{Deserialize, Serialize};

/// The four cardinal points of a compass.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize, Hash, Eq)]
pub enum CardinalDirection {
    /// The north direction.
    North,
    /// The east direction.
    East,
    /// The south direction.
    South,
    /// The west direction.
    West,
}

/// The four cardinal points of a compass.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize, Hash, Eq)]
pub enum OrdinalDirection {
    /// North east direction
    NorthEast,
    /// South east direction
    SouthEast,
    /// South west direction
    SouthWest,
    /// North west direction
    NorthWest,
}

/// The eight combined cardinal and ordinal directions.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize, Hash, Eq)]
pub enum CompassDirection {
    /// North direction
    North,
    /// North east direction
    NorthEast,
    /// East direction
    East,
    /// South east direction
    SouthEast,
    /// The south direction.
    South,
    /// South west direction
    SouthWest,
    /// The west direction.
    West,
    /// North west direction
    NorthWest,
}

impl Into<CompassDirection> for CardinalDirection {
    fn into(self) -> CompassDirection {
        match self {
            CardinalDirection::North => CompassDirection::North,
            CardinalDirection::East => CompassDirection::East,
            CardinalDirection::South => CompassDirection::South,
            CardinalDirection::West => CompassDirection::West,
        }
    }
}

impl Into<CompassDirection> for OrdinalDirection {
    fn into(self) -> CompassDirection {
        match self {
            OrdinalDirection::NorthEast => CompassDirection::NorthEast,
            OrdinalDirection::SouthEast => CompassDirection::SouthEast,
            OrdinalDirection::SouthWest => CompassDirection::SouthWest,
            OrdinalDirection::NorthWest => CompassDirection::NorthWest,
        }
    }
}

impl PartialEq<CompassDirection> for CardinalDirection {
    fn eq(&self, other: &CompassDirection) -> bool {
        match (self, other) {
            (CardinalDirection::North, CompassDirection::North) => true,
            (CardinalDirection::East, CompassDirection::East) => true,
            (CardinalDirection::South, CompassDirection::South) => true,
            (CardinalDirection::West, CompassDirection::West) => true,
            _ => false,
        }
    }
}

impl PartialEq<CompassDirection> for OrdinalDirection {
    fn eq(&self, other: &CompassDirection) -> bool {
        match (self, other) {
            (OrdinalDirection::NorthEast, CompassDirection::NorthEast) => true,
            (OrdinalDirection::SouthEast, CompassDirection::SouthEast) => true,
            (OrdinalDirection::SouthWest, CompassDirection::SouthWest) => true,
            (OrdinalDirection::NorthWest, CompassDirection::NorthWest) => true,
            _ => false,
        }
    }
}

//TODO: Impliment conversion to bearing/heading.