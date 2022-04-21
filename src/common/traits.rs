use crate::{Aabr, Point, AxisValue, Collision, PrimaNum};

/// Used for implimenting 'fast' distance calculations.
/// This is useful when we can't use square root on the generic type 'N', but
/// still need to gauge the distance.
pub trait FastDistance<Rhs = Self> {
    /// The output value.
    type Output;
    /// Computes the manhattan distance between two points. While this isn't a
    /// squared distance, it is also a good workaround for non-squarable types.
    fn manhatten_distance(&self, other: &Rhs) -> Self::Output;
    /// Computes the squared euclidean distance between two points.
    fn distance_squared(&self, other: &Rhs) -> Self::Output;
}

/// A trait that represents distance between two points. Requires implimentation of
/// [FastDistance].
pub trait Distance<Rhs = Self>: FastDistance<Rhs> {
    /// Computes the euclidean distance between two points.
    fn distance(&self, other: &Rhs) -> Self::Output;
}

/// Cross product for points, floats and other such things.
pub trait Cross<Rhs = Self> {
    /// The product of the cross.
    type Product;
    /// Returns the cross product of self and other.
    fn cross_product(&self, other: &Rhs) -> Self::Product;
}

/// A trait that implements the dot product of two points.
pub trait Dot<Rhs = Self> {
    /// The output value.
    type Output;
    /// Computes the dot product of two values.
    fn dot(&self, other: &Rhs) -> Self::Output;
}

/// A trait that ensures that the type is a point.
pub trait Coordinate<N>
where
    N: num_traits::Num,
{
    /// The axis this point type covers.
    fn axis(&self) -> AxisValue<N>;
}

/// A two dimensional shape that can be used for collision detection.
pub trait Shape<N>
where
    N: PrimaNum,
{
    /// The area of the shape.
    fn area(&self) -> N;
    /// The circumference of the shape.
    fn circumference(&self) -> N;
    /// The center of the shape.
    fn center(&self) -> Point<N>;
    /// A bounding box that contains the shape.
    fn bounding_box(&self) -> Aabr<N>;
    /// Returns true if the shape contains the point.
    fn contains_point(&self, point: &Point<N>) -> bool;
}

/// A trait that enforces certian behaviors between two shapes.
pub trait Interact<N, Rhs = Self> {
    /// Computes the collision between two geometric objects.
    fn collision(&self, other: &Rhs) -> Option<Collision<N>>;

    /// Gets the nearest point of 'other' to 'self'.
    fn nearest_extent(&self, other: &Rhs) -> Point<N>;

    /// Returns true if the two objects collide. This should always produce the same result as calling [intersecting()] on the two objects.
    /// ### Note
    /// While this acheives the same as calling [intersecting()] on the two objects, it is probably faster to use the latter as this uses 
    /// the more complex opperation of [collision()] as an intermediate step (when impliemented as default).
    fn colliding(&self, other: &Rhs) -> bool {
        self.collision(other).is_some()
    }
}

/// A trait to check if two shapes are intersecting. For more complex interactions between shapes, use the trait [Interact].
pub trait Intersect<Rhs = Self> {
    /// Computes the intersection between two geometric objects.
    fn intersecting(&self, other: &Rhs) -> bool;
}
