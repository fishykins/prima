use crate::{
    Distance, FastDistance, Interact, Intersect, Point, PrimaFloat, PrimaNum, Shape, Vector,
};
use serde::{Deserialize, Serialize};

/// A simple circle, defined by a center and radius.
/// # Examples
///
/// ```
/// let circle = Circle::new(Point::new(0.0, 0.0), 32.0);
/// ```
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct Circle<N = super::DefaultFloat> {
    /// The middle of the circle.
    pub center: Point<N>,
    /// Radius of the circle.
    pub radius: N,
}

impl<N> Circle<N> {
    /// Builds a circle from given center and radius.
    pub fn new(center: Point<N>, radius: N) -> Self {
        Self { center, radius }
    }
}

impl<N> FastDistance for Circle<N>
where
    N: PrimaNum,
{
    type Output = N;

    /// Returns the distance between two circles. If the circles are intersecting, the distance is less than 0.
    /// This could potentially cause 'Subtraction with overflow" errors, so be careful if using unsigned types.
    fn manhatten_distance(&self, other: &Self) -> Self::Output {
        let x = if self.center.x > other.center.x {
            self.center.x - other.center.x
        } else {
            other.center.x - self.center.x
        };
        let y = if self.center.y > other.center.y {
            self.center.y - other.center.y
        } else {
            other.center.y - self.center.y
        };

        x + y - self.radius - other.radius
    }

    // NOTE: Is this any good? Ask a mathematician.
    fn distance_squared(&self, other: &Self) -> Self::Output {
        let d = self.center.distance_squared(&other.center);
        let r = (self.radius + other.radius) * (self.radius + other.radius);
        d - r
    }
}

impl<N> Distance for Circle<N>
where
    N: PrimaFloat,
{
    fn distance(&self, other: &Self) -> Self::Output {
        let d = self.center.distance(&other.center);
        let r = self.radius + other.radius;
        d - r
    }
}

impl<N> Shape<N> for Circle<N>
where
    N: PrimaFloat,
{
    fn area(&self) -> N {
        N::pi() * self.radius * self.radius
    }

    fn circumference(&self) -> N {
        (N::pi() + N::pi()) * self.radius
    }

    fn center(&self) -> Point<N> {
        self.center
    }

    fn bounding_box(&self) -> crate::Aabr<N> {
        crate::Aabr::new(
            self.center - Point::splat(self.radius),
            self.center + Point::splat(self.radius),
        )
    }

    fn contains_point(&self, point: &Point<N>) -> bool {
        self.center.distance_squared(point) <= self.radius * self.radius
    }

    fn nearest_point(&self, point: &Point<N>) -> Point<N> {
        let d = self.center.distance(point);
        let r = self.radius;
        if d > r {
            self.center + (point.vector(self.center)).normalize() * r
        } else {
            self.center
        }
    }
}

impl<N> Interact<N> for Circle<N>
where
    N: PrimaFloat,
{
    fn collision(&self, other: &Self) -> Option<crate::Collision<N>> {
        let d = self.center.distance(&other.center);
        let r = self.radius + other.radius;
        if d <= r {
            let normal: Vector<N> = (other.center - self.center).into();
            if normal == Vector::zero() {
                Some(crate::Collision {
                    // Pseudo random (but predictable) values for when the circles are one.
                    penetration: self.radius,
                    normal: Vector::new(N::one(), N::zero()),
                })
            } else {
                let penetration = r - d;
                Some(crate::Collision {
                    penetration,
                    normal,
                })
            }
        } else {
            None
        }
    }

    fn nearest_extent(&self, other: &Self) -> Point<N> {
        let v: Vector<N> = (other.center - self.center).into();
        let normalized = v.normalize();
        other.center + normalized * other.radius
    }
}

impl<N> Intersect for Circle<N>
where
    N: PrimaNum,
{
    fn intersecting(&self, other: &Self) -> bool {
        self.center.distance_squared(&other.center)
            <= (self.radius + other.radius) * (self.radius + other.radius)
    }
}
