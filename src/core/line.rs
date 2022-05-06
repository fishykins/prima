use super::{Point, Vector};
use crate::{
    nums::{PrimaFloat, PrimaNum},
    traits::{Cross, Distance, Magnitude, Nearest},
};

/// A line between two points.
pub struct Line<N = f32> {
    /// The starting point of the line.
    pub start: Point<N>,
    /// The ending point of the line.
    pub end: Point<N>,
}

impl<N> Line<N>
where
    N: PrimaNum,
{
    /// Creates a new line.
    pub fn new(start: Point<N>, end: Point<N>) -> Line<N> {
        Line { start, end }
    }

    /// Creates a new line using a point and a vector.
    pub fn from_point(p: Point<N>, v: Vector<N>) -> Line<N> {
        Line {
            start: p,
            end: p + v,
        }
    }
}

impl<N> Line<N>
where
    N: PrimaFloat,
{
    /// Returns the point of collision between the two lines.
    pub fn collision(&self, other: &Self) -> Option<Point<N>> {
        let denom: N = self.vector().cross(&other.vector());
        if denom == N::zero() {
            return None;
        }

        let numer_a = (other.start - self.start).cross(&other.vector());
        let numer_c = (other.start - self.start).cross(&self.vector());

        let t = numer_a / denom;
        let u = numer_c / denom;

        if t < N::zero() || t > N::one() || u < N::zero() || u > N::one() {
            return None;
        }
        return Some(self.start + self.vector() * t);
    }

    /// Returns the line's vector.
    pub fn vector(&self) -> Vector<N> {
        self.end - self.start
    }

    /// Returns the line's normal. This is facing away from the line, 90 degrees to the left.
    pub fn normal(&self) -> Vector<N> {
        self.vector().perpendicular_cc().normalize()
    }

    /// Gets the relative dot product of the point along the line.
    pub fn relative_dot(&self, p: &Point<N>) -> N {
        (*p - self.start).dot(&self.vector()) / self.magnitude_squared()
    }

    /// Projects the given point onto an unbound self.
    pub fn project_point(&self, p: &Point<N>) -> Point<N> {
        let v = self.relative_dot(p);
        self.start + self.vector() * v
    }
}

impl<N> Magnitude<N> for Line<N>
where
    N: PrimaFloat,
{
    fn magnitude_squared(&self) -> N {
        let dx = (self.end.x - self.start.x).abs();
        let dy = (self.end.y - self.start.y).abs();
        dx * dx + dy * dy
    }
}

//=================================================================//
//========================= POINT =================================//
//=================================================================//

impl<N> Distance<N, Point<N>> for Line<N>
where
    N: PrimaFloat,
{
    fn squared_distance(&self, point: &Point<N>) -> N {
        let v = self.relative_dot(point).clamp_01();
        let p = self.start + self.vector() * v;
        p.squared_distance(point)
    }
}

impl<N> Nearest<N, Point<N>> for Line<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, point: &Point<N>) -> Point<N> {
        let dist = self.relative_dot(point);

        if dist < N::zero() {
            self.start
        } else if dist > N::one() {
            self.end
        } else {
            self.start + self.vector() * dist
        }
    }
}

//=================================================================//
//============================= LINE ==============================//
//=================================================================//

impl<N> Distance<N, Line<N>> for Line<N>
where
    N: PrimaFloat,
{
    fn squared_distance(&self, other: &Line<N>) -> N {
        if self.collision(&other).is_some() {
            return N::zero();
        }
        let a = self.nearest_point(other);
        let b = other.nearest_point(&a);
        a.squared_distance(&b)
    }
}

impl<N> Nearest<N, Line<N>> for Line<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, other: &Line<N>) -> Point<N> {
        let (a, b) = (self, other);
        // First, get alignment in the direction of this line.
        // The line 'self' is treated as being horizontal from left to right.
        let x_a = a.relative_dot(&b.start);
        let x_b = a.relative_dot(&b.end);
        let x_overlap = x_a.is_decimal()
            || x_b.is_decimal()
            || (x_a >= N::one() && x_b <= N::zero())
            || (x_b >= N::one() && x_a <= N::zero());

        // Check the normal axis to see if 'other' intersects our line's direction.
        let y_a = (b.start - self.start).dot(&self.normal()) / self.magnitude_squared();
        let y_b = (b.end - self.start).dot(&self.normal()) / self.magnitude_squared();
        let y_overlap = y_a.signum() != y_b.signum() && !y_a.is_zero() && !y_b.is_zero();

        // println!("x_a: {}, x_b: {} => {}", x_a, x_b, x_overlap);
        // println!("y_a: {}, y_b: {} => {}", y_a, y_b, y_overlap);

        if !y_overlap {
            // 'other' is only on one side of our line, which is good.
            if !x_overlap {
                // 'other' is way out of scope, so we can return start or end.
                if x_a + x_b > N::one() {
                    a.end
                } else {
                    a.start
                }
            } else {
                if y_a.abs() <= y_b.abs() && x_a.is_decimal() {
                    // other.start is in range and closest to self, so use that to finish.
                    a.start + a.vector() * x_a
                } else {
                    // Assuming all prior logic is correct, there can only be one case here:
                    // end is in a valid position and is closer to the line than start.
                    a.start + a.vector() * x_b
                }
            }
        } else {
            // Normal is crossing, so we need to check the other axis.
            if x_a > N::zero() && x_b >= N::zero() {
                self.end
            } else {
                self.start
            }
        }
    }
}
