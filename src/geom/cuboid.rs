use super::{Float, Line1, Vec3};
use crate::core::Axis;

/// Axis aligned cuboid
/// # Examples
///
/// ```
/// let cuboid = Cuboid::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(8.0, 8.0, 4.0));
/// assert_eq!(rect.width(), 8.0);
/// assert_eq!(rect.height(), 8.0);
/// assert_eq!(rect.length(), 4.0);
/// ```
#[derive(Clone, Copy)]
pub struct Cuboid {
    /// The min point of the Rect.
    pub min: Vec3,
    /// The max point of the Rect.
    pub max: Vec3,
}

impl Cuboid {
    /// Creates a new unvalidated Cuboid from given min and max points.
    /// # Example
    /// ```
    /// let cuboid = Cuboid::new(Vec3::ZERO, Vec3::splat(8.0));
    /// assert!(cuboid.valid());
    ///
    /// let cuboid2 = Cuboid::new(Vec3::splat(8.0), Vec3::ZERO);
    /// assert!(!cuboid2.valid());
    /// ```
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// Creates a new validated Rect from given min and max points.
    /// # Example
    /// ```
    /// let rect = Rect::new(Vec2::ZERO, Vec2::splat(8.0));
    /// assert!(rect.valid());
    ///
    /// let rect2 = Rect::new(Vec2::splat(8.0), Vec2::ZERO);
    /// assert!(rect2.valid());
    /// ```
    pub fn new_valid(min: Vec3, max: Vec3) -> Self {
        Self { min, max }.validate()
    }

    /// Checks if the Cuboid has valid min and max points.
    pub fn valid(&self) -> bool {
        self.min.x < self.max.x && self.min.y < self.max.y && self.min.z < self.max.z
    }

    /// Converts the Cuboid into a validated version of itself.
    pub fn validate(self) -> Self {
        let (x1, x2) = if self.min.x < self.max.x {
            (self.min.x, self.max.x)
        } else {
            (self.max.x, self.min.x)
        };
        let (y1, y2) = if self.min.y < self.max.y {
            (self.min.y, self.max.y)
        } else {
            (self.max.y, self.min.y)
        };
        let (z1, z2) = if self.min.z < self.max.z {
            (self.min.z, self.max.z)
        } else {
            (self.max.z, self.min.z)
        };
        return Self {
            min: Vec3::new(x1, y1, z1),
            max: Vec3::new(x2, y2, z2),
        };
    }

    /// Returns the width of the Cuboid.
    pub fn width(&self) -> Float {
        self.max.x - self.min.x
    }

    /// Returns the height of the Cuboid.
    pub fn height(&self) -> Float {
        self.max.y - self.min.y
    }

    /// Returns the length of the Cuboid.
    pub fn length(&self) -> Float {
        self.max.z - self.min.z
    }

    /// Calculates the central point of the Cuboid.
    pub fn center(&self) -> Vec3 {
        (self.min + self.max) / 2.0
    }

    /// Returns all eight corners of this Cuboid.
    pub fn corners(&self) -> [Vec3; 8] {
        let x1 = self.min.x;
        let x2 = self.max.x;
        let y1 = self.min.y;
        let y2 = self.max.y;
        let z1 = self.min.z;
        let z2 = self.max.z;
        [
            Vec3::new(x1, y1, z1),
            Vec3::new(x2, y1, z1),
            Vec3::new(x1, y2, z1),
            Vec3::new(x2, y2, z1),
            Vec3::new(x1, y1, z2),
            Vec3::new(x2, y1, z2),
            Vec3::new(x1, y2, z2),
            Vec3::new(x2, y2, z2),
        ]
    }

    /// Returns [`true`] if the Cuboid contains the point 'p'.
    pub fn contains_point(&self, p: Vec3) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }

    /// Returns [`true`] if this contains the given Cuboid.
    pub fn contains_cuboid(&self, other: &Self) -> bool {
        other.min.x >= self.min.x
            && other.min.x <= self.max.x
            && other.max.x >= self.min.x
            && other.max.x <= self.max.x
            && other.min.y >= self.min.y
            && other.min.y <= self.max.y
            && other.max.y >= self.min.y
            && other.max.y <= self.max.y
            && other.min.z >= self.min.z
            && other.min.z <= self.max.z
            && other.max.z >= self.min.z
            && other.max.z <= self.max.z
    }

    /// Splits this into two new Cuboid along the provided axis, lerped by position.
    /// # Example
    /// ```
    /// let rect = Rect::new(Vec2::ZERO, Vec2::ONE);
    /// let (a, b) = rect.split(0.5, Axis::Vertical);
    /// ```
    pub fn split(self, _position: Float, _axis: Axis) -> (Self, Self) {
        todo!()
    }

    /// Splits along the x axis.
    pub fn split_x(self, _position: Float) -> (Self, Self) {
        todo!()
    }

    /// Splits along the y axis.
    pub fn split_y(self, _position: Float) -> (Self, Self) {
        todo!()
    }

    /// Determines if two Cuboid have overlapping bounds.
    pub fn intersects(&self, other: &Self) -> bool {
        self.x_intersects(other) && self.y_intersects(other)
    }

    /// Returns [`true`] if the other Cuboid overlaps on the x axis.
    pub fn x_intersects(&self, other: &Self) -> bool {
        !(self.min.x > other.max.x || other.min.x > self.max.x)
    }

    /// Returns [`true`] if the other Cuboid overlaps on the y axis.
    pub fn y_intersects(&self, other: &Self) -> bool {
        !(self.min.y > other.max.y || other.min.y > self.max.y)
    }

    /// Returns [`true`] if the other Cuboid overlaps on the z axis.
    pub fn z_intersects(&self, other: &Self) -> bool {
        !(self.min.z > other.max.z || other.min.z > self.max.z)
    }

    /// Returns the Cuboid that contains the intersection of the two Cuboid,
    /// or none if no intersection exists.
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        if !self.intersects(other) {
            return None;
        }
        // Maximum of mins
        let min_x = if self.min.x > other.min.x {
            self.min.x
        } else {
            other.min.x
        };
        let min_y = if self.min.y > other.min.y {
            self.min.y
        } else {
            other.min.y
        };
        let min_z = if self.min.z > other.min.z {
            self.min.z
        } else {
            other.min.z
        };

        // Minimum of maxs
        let max_x = if self.max.x < other.max.x {
            self.max.x
        } else {
            other.max.x
        };
        let max_y = if self.max.y < other.max.y {
            self.max.y
        } else {
            other.max.y
        };
        let max_z = if self.max.z < other.max.z {
            self.max.z
        } else {
            other.max.z
        };

        Some(Cuboid::new(
            Vec3::new(min_x, min_y, min_z),
            Vec3::new(max_x, max_y, max_z),
        ))
    }

    /// Returns the intersection on the x axis, if there is any.
    pub fn x_intersection(&self, other: &Self) -> Option<Line1> {
        if !self.x_intersects(other) {
            return None;
        }
        let min = if self.min.x > other.min.x {
            self.min.x
        } else {
            other.min.x
        };
        let max = if self.max.x < other.max.x {
            self.max.x
        } else {
            other.max.x
        };
        Some(Line1::new(min, max))
    }

    /// Returns the intersection on the y axis, if there is any.
    pub fn y_intersection(&self, other: &Cuboid) -> Option<Line1> {
        if !self.y_intersects(other) {
            return None;
        }
        let min = if self.min.y > other.min.y {
            self.min.y
        } else {
            other.min.y
        };
        let max = if self.max.y < other.max.y {
            self.max.y
        } else {
            other.max.y
        };
        Some(Line1::new(min, max))
    }

    /// Returns the intersection on the z axis, if there is any.
    pub fn z_intersection(&self, other: &Cuboid) -> Option<Line1> {
        if !self.z_intersects(other) {
            return None;
        }
        let min = if self.min.z > other.min.z {
            self.min.z
        } else {
            other.min.z
        };
        let max = if self.max.z < other.max.z {
            self.max.z
        } else {
            other.max.z
        };
        Some(Line1::new(min, max))
    }
}
