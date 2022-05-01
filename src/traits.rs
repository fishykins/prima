use crate::{shapes::{Circle, Aabr}, nums::PrimaFloat, core::*};
///=============================================================///
///======================= LOCAL TRAITS ========================///
///=============================================================///

/// The main trait for a geometric object.
pub trait Shape<N> {
    /// The object's volume.
    fn volume(&self) -> N;
    /// The circumference of the shape.
    fn circumference(&self) -> N;
    /// The bounding rect of this shape.
    fn bounding_rect(&self) -> Aabr<N>;
    /// The bounding circle of this shape.
    fn bounding_circle(&self) -> Circle<N>;
}

/// A shape-object that has flat edges.
pub trait Flat<N>: Shape<N> {
    /// Returns vertices of the object.
    fn vertices(&self) -> Vec<Point<N>>;
    /// Returns edges of the object.
    fn edges(&self) -> Vec<Line<N>>;
}

/// A shape with curved edges.
pub trait Curved<N>: Shape<N> {}

/// A shape that can be rotated.
pub trait LocalRotation<N>: Shape<N> {
    /// Returns the rotation of the shape.
    fn rotation(&self) -> Angle<N>;
}

/// A shape that has positional data.
pub trait LocalPosition<N>: Shape<N> {
    /// Returns the position of the shape.
    fn position(&self) -> Point<N>;
}

/// Something that can have magnitude/length.
pub trait Magnitude<N>
where
    N: PrimaFloat,
{
    /// The magnitude of the object.
    fn magnitude(&self) -> N {
        self.magnitude_squared().sqrt()
    }
    /// The squared magnitude of the object.
    fn magnitude_squared(&self) -> N;
}

///=============================================================///
///======================= INTERACTIONS ========================///
///=============================================================///

/// Base trait for distance between two entities.
pub trait Distance<N, Rhs = Point<N>>
where
    N: PrimaFloat,
{
    /// Returns the square distance between two objects.
    fn squared_distance(&self, other: &Rhs) -> N;

    /// Returns the distance between two objects.
    fn distance(&self, other: &Rhs) -> N {
        self.squared_distance(other).sqrt()
    }
}


/// Calculating nearest extents.
pub trait Nearest<N, Rhs = Self>: Distance<N, Rhs>
where
    N: PrimaFloat,
{
    /// Returns the nearest point on self to the given object.
    fn nearest_extent_to_other(&self, other: &Rhs) -> Point<N>;
    /// Returns the nearest point on other to self.
    fn nearest_extent_to_self(&self, other: &Rhs) -> Point<N>;

    /// Returns the shortest line between two objects.
    fn shortest_line(&self, other: &Rhs) -> Line<N> {
        let nearest_self = self.nearest_extent_to_other(other);
        let nearest_other = self.nearest_extent_to_self(other);
        Line::new(nearest_self, nearest_other)
    }
}

/// A trait for objects that can collide with other objects.
pub trait Collide<N, Rhs = Self>: Nearest<N, Rhs>
where
    N: PrimaFloat,
{
    /// Returns the collision response of the object.
    fn collision(&self, other: &Rhs) -> Option<Collision<N>>;
    /// Checks if the object collides with another object.
    fn intersecting(&self, other: &Rhs) -> bool {
        self.collision(other).is_some()
    }
    /// Checks if this object entirely contains the other object.
    fn enveloping(&self, other: &Rhs) -> bool;
    /// Checks if this object is entirely contained by the other object.
    fn enveloped_by(&self, other: &Rhs) -> bool;
}