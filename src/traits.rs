use crate::{
    core::*,
    nums::{PrimaFloat, PrimaNum},
    shapes::{Aabr, Circle},
};
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
    /// Returns true if the given point is inside the shape.
    fn contains(&self, point: &Point<N>) -> bool;
}

/// A shape-object that has flat edges.
pub trait Flat<N>: Shape<N>
where
    N: PrimaNum,
{
    /// Returns vertices of the object.
    fn vertices(&self) -> Vec<Point<N>>;
    /// Returns edges of the object.
    fn edges(&self) -> Vec<Line<N>> {
        let mut edges = Vec::new();
        let vertices = self.vertices();
        for i in 0..vertices.len() {
            let next = if i == vertices.len() - 1 { 0 } else { i + 1 };
            edges.push(Line::new(vertices[i], vertices[next]));
        }
        edges
    }
}

/// A shape with curved edges.
pub trait Curved<N>: Shape<N> {}

/// A shape that can be rotated.
pub trait LocalRotation<N>: Shape<N> + LocalPosition<N>
where
    N: PrimaFloat,
{
    /// Returns the rotation of the shape.
    fn rotation(&self) -> Angle<N>;
    /// Rotates the shape by the given amount.
    fn rotate(&mut self, rotation: Rotation<N>);
    /// Rotate around a point.
    fn rotate_around(&mut self, rotation: Rotation<N>, point: Point<N>) {
        let v = self.position() - point;
        let new_center = point + v * rotation;
        let offset = new_center - self.position();
        self.translate(&offset);
        self.rotate(rotation);
    }
}

/// A shape that has positional data.
pub trait LocalPosition<N>: Shape<N> {
    /// Returns the position of the shape.
    fn position(&self) -> Point<N>;
    /// Translates the shape.
    fn translate(&mut self, offset: &Vector<N>);
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

/// Cross product for points, floats and other such things.
pub trait Cross<Rhs = Self> {
    /// The product of the cross.
    type Product;
    /// Returns the cross product of self and other.
    fn cross(&self, other: &Rhs) -> Self::Product;
}


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
pub trait Nearest<N, Rhs = Self>
where
    N: PrimaFloat,
{
    /// Returns the nearest point on self to the given object.
    fn nearest_point(&self, other: &Rhs) -> Point<N>;
}

/// A trait for objects that can collide with other objects.
pub trait Collide<N, Rhs = Self>
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