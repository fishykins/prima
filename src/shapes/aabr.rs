use crate::{
    core::{project_shape_to_axis_pair, Collision, Extent, Line, Point, Vector},
    nums::{PrimaFloat, PrimaNum},
    traits::{Collide, Distance, Flat, LocalPosition, Nearest, Shape},
};

use super::{Circle, Obr};

/// An axis-aligned bounding rectangle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Aabr<N> {
    /// The minimum point of the rectangle.
    pub min: Point<N>,
    /// The maximum point of the rectangle.
    pub max: Point<N>,
}

impl<N> Aabr<N>
where
    N: PrimaNum,
{
    /// Creates a new Aabr from a center point, width and height.
    pub fn from_point(center: Point<N>, width: N, height: N) -> Self {
        let e = Extent::<N>::new(width, height);
        Self {
            min: center - e,
            max: center + e,
        }
    }

    /// Creates a new Aabr from a min and max point.
    pub fn new(min: Point<N>, max: Point<N>) -> Self {
        Self { min, max }
    }

    /// Returns the min max pair.
    pub fn min_max(&self) -> (Point<N>, Point<N>) {
        (self.min, self.max)
    }

    /// Returns the width of the Aabr.
    pub fn width(&self) -> N {
        self.max.x - self.min.x
    }

    /// Returns the height of the Aabr.
    pub fn height(&self) -> N {
        self.max.y - self.min.y
    }

    /// Returns the extent of this Aabr.
    pub fn extent(&self) -> Extent<N> {
        Extent::new(self.width(), self.height())
    }

    /// Returns any overlap between the two bounding boxes.
    pub fn overlap(&self, other: &Self) -> Option<Self> {
        let (min_a, max_a) = self.min_max();
        let (min_b, max_b) = other.min_max();
        if min_a.x > max_b.x || max_a.x < min_b.x || min_a.y > max_b.y || max_a.y < min_b.y {
            return None;
        }
        Some(self.overlap_unchecked(other))
    }

    /// Returns the overlap of two bounding boxes, without checking for validity.
    pub fn overlap_unchecked(&self, other: &Self) -> Self {
        let (min_a, max_a) = self.min_max();
        let (min_b, max_b) = other.min_max();

        // This looks verbose, but it allows us to avoid requiring Ord for N.
        let min_x = if min_a.x > min_b.x { min_a.x } else { min_b.x };
        let min_y = if min_a.y > min_b.y { min_a.y } else { min_b.y };
        let max_x = if max_a.x < max_b.x { max_a.x } else { max_b.x };
        let max_y = if max_a.y < max_b.y { max_a.y } else { max_b.y };

        Self::new(Point::new(min_x, min_y), Point::new(max_x, max_y))
    }
}

impl<N> Shape<N> for Aabr<N>
where
    N: PrimaFloat,
{
    fn volume(&self) -> N {
        self.extent().volume()
    }

    fn circumference(&self) -> N {
        self.extent().double().sum()
    }

    fn bounding_rect(&self) -> Aabr<N> {
        self.clone()
    }

    fn bounding_circle(&self) -> Circle<N> {
        let center = self.position();
        let radius = center.distance(&self.max);
        Circle::new(center, radius)
    }

    fn contains(&self, point: &Point<N>) -> bool {
        let (min, max) = self.min_max();
        point.x >= min.x && point.x <= max.x && point.y >= min.y && point.y <= max.y
    }
}

impl<N> Flat<N> for Aabr<N>
where
    N: PrimaFloat,
{
    fn vertices(&self) -> Vec<Point<N>> {
        let min = self.min;
        let max = self.max;
        [
            Point::new(min.x, min.y),
            Point::new(min.x, max.y),
            Point::new(max.x, max.y),
            Point::new(max.x, min.y),
        ]
        .into()
    }
}

impl<N> LocalPosition<N> for Aabr<N>
where
    N: PrimaFloat,
{
    fn position(&self) -> Point<N> {
        Point::new(self.min.x + self.max.x, self.min.y + self.max.y) / (N::one() + N::one())
    }

    fn translate(&mut self, offset: &Vector<N>) {
        self.min += *offset;
        self.max += *offset;
    }
}

//=================================================================//
//========================= POINT =================================//
//=================================================================//

impl<N> Distance<N, Point<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn distance_squared(&self, other: &Point<N>) -> N {
        self.nearest_point(other).distance_squared(other)
    }
}

impl<N> Nearest<N, Point<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, point: &Point<N>) -> Point<N> {
        let mut nearest = self.position();
        let min = self.min;
        let max = self.max;
        if point.x < min.x {
            nearest.x = min.x;
        } else if point.x > max.x {
            nearest.x = max.x;
        }
        if point.y < min.y {
            nearest.y = min.y;
        } else if point.y > max.y {
            nearest.y = max.y;
        }
        nearest
    }
}

//=================================================================//
//============================= LINE ==============================//
//=================================================================//

impl<N> Distance<N, Line<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn distance_squared(&self, line: &Line<N>) -> N {
        let a = self.nearest_point(line);
        let b = line.nearest_point(&a);
        a.distance_squared(&b)
    }
}

impl<N> Nearest<N, Line<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, line: &Line<N>) -> Point<N> {
        let start_x = (line.start.x - self.min.x) / self.width();
        let start_y = (line.start.y - self.min.y) / self.height();
        let end_x = (line.end.x - self.min.x) / self.width();
        let end_y = (line.end.y - self.min.y) / self.height();
        let line_min = Point::new(start_x.min(end_x), start_y.min(end_y));
        let line_max = Point::new(start_x.max(end_x), start_y.max(end_y));

        let x = if line_min.x >= N::one() {
            Some(self.max.x)
        } else if line_max.x <= N::zero() {
            Some(self.min.x)
        } else {
            None
        };

        let y = if line_min.y >= N::one() {
            Some(self.max.y)
        } else if line_max.y <= N::zero() {
            Some(self.min.y)
        } else {
            None
        };

        let right_slant = line.start.x < line.end.x;
        let up_slant = line.start.y < line.end.y;

        if let Some(x) = x {
            if let Some(y) = y {
                Point::new(x, y)
            } else {
                let y = if right_slant {
                    line.end.y.max(self.min.y).min(self.max.y)
                } else {
                    line.start.y.max(self.min.y).min(self.max.y)
                };
                Point::new(x, y)
            }
        } else {
            if let Some(y) = y {
                let x = if up_slant && y <= self.min.y || !up_slant && y >= self.max.y {
                    line.end.x.max(self.min.x).min(self.max.x)
                } else {
                    line.start.x.max(self.min.x).min(self.max.x)
                };
                Point::new(x, y)
            } else {
                // solve for a corner. 
                if right_slant {
                    if up_slant {
                        Point::new(self.min.x, self.max.y)
                    } else {
                        Point::new(self.min.x, self.min.y)
                    }
                } else {
                    if up_slant {
                        Point::new(self.max.x, self.max.y)
                    } else {
                        Point::new(self.max.x, self.min.y)
                    }
                }
            }
        }
    }
}

impl<N> Collide<N, Line<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn collision(&self, line: &Line<N>) -> Option<Collision<N>> {
        let n = line.nearest_point(&self.position());
        if !self.contains(&n) {
            return None;
        }
        let normal = line.normal();
        let x_overlap = (self.max.x - n.x).min(n.x - self.min.x);
        let y_overlap = (self.max.y - n.y).min(n.y - self.min.y);

        Some(Collision::new(n, normal, x_overlap.min(y_overlap)))
    }

    fn enveloping(&self, line: &Line<N>) -> bool {
        self.contains(&line.start) && self.contains(&line.end)
    }

    fn enveloped_by(&self, _: &Line<N>) -> bool {
        false
    }
}

//=================================================================//
//============================ CIRCLE =============================//
//=================================================================//

impl<N> Distance<N, Circle<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn distance_squared(&self, other: &Circle<N>) -> N {
        self.nearest_point(&other.center)
            .distance_squared(&other.center)
    }
}

impl<N> Nearest<N, Circle<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, circle: &Circle<N>) -> Point<N> {
        self.nearest_point(&circle.center)
    }
}

impl<N> Collide<N, Circle<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn collision(&self, circle: &Circle<N>) -> Option<Collision<N>> {
        let n = circle.nearest_point(self);
        if !circle.contains(&n) {
            return None;
        }
        let depth = circle.radius - n.distance(&circle.center);
        let normal = n - circle.center;

        // TODO: Resolve when circle center is within the aabr. The center of the circle needs to be clipped to the closest edge of the Aabr, and the normal needs to be flipped.

        Some(Collision::new(n, normal, depth))
    }

    fn intersecting(&self, circle: &Circle<N>) -> bool {
        if self.contains(&circle.center) {
            return true;
        }
        let aabr_center = self.position();
        let two = N::one() + N::one();
        let half_width = self.width() / two;
        let half_height = self.height() / two;

        let circle_distance_x = (circle.center.x - aabr_center.x).abs();
        let circle_distance_y = (circle.center.y - aabr_center.y).abs();

        if circle_distance_x > half_width + circle.radius {
            return false;
        }
        if circle_distance_y > half_height + circle.radius {
            return false;
        }

        if circle_distance_x <= half_width {
            return true;
        }
        if circle_distance_y <= half_height {
            return true;
        }

        let corner_dist_sq =
            (circle_distance_x - half_width).powi(2) + (circle_distance_y - half_height).powi(2);

        corner_dist_sq <= circle.radius * circle.radius
    }

    fn enveloping(&self, circle: &Circle<N>) -> bool {
        let aabr_center = self.position();
        let extent = self.extent().half();
        let half_width = extent.width();
        let half_height = extent.height();

        let circle_distance_x = (circle.center.x - aabr_center.x).abs();
        let circle_distance_y = (circle.center.y - aabr_center.y).abs();

        if circle_distance_x > half_width + circle.radius {
            return false;
        }
        if circle_distance_y > half_height + circle.radius {
            return false;
        }

        if circle_distance_x <= half_width {
            return true;
        }
        if circle_distance_y <= half_height {
            return true;
        }

        let corner_dist_sq =
            (circle_distance_x - half_width).powi(2) + (circle_distance_y - half_height).powi(2);

        corner_dist_sq <= circle.radius * circle.radius
    }

    fn enveloped_by(&self, circle: &Circle<N>) -> bool {
        self.vertices().iter().all(|v| circle.contains(v))
    }
}

//=================================================================//
//============================= AABR ==============================//
//=================================================================//

impl<N> Distance<N, Aabr<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn distance_squared(&self, other: &Aabr<N>) -> N {
        let (min_a, max_a) = self.min_max();
        let (min_b, max_b) = other.min_max();
        let mut d = N::zero();
        if min_a.x > max_b.x {
            d += (min_a.x - max_b.x).powi(2);
        } else if max_a.x < min_b.x {
            d += (max_a.x - min_b.x).powi(2);
        }
        if min_a.y > max_b.y {
            d += (min_a.y - max_b.y).powi(2);
        } else if max_a.y < min_b.y {
            d += (max_a.y - min_b.y).powi(2);
        }
        d
    }
}

impl<N> Nearest<N, Aabr<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, other: &Aabr<N>) -> Point<N> {
        let mut nearest = self.position();
        let min = self.min;
        let max = self.max;
        let (min_o, max_o) = other.min_max();
        if min.x < min_o.x {
            nearest.x = min_o.x;
        } else if max.x > max_o.x {
            nearest.x = max_o.x;
        }
        if min.y < min_o.y {
            nearest.y = min_o.y;
        } else if max.y > max_o.y {
            nearest.y = max_o.y;
        }
        nearest
    }
}

impl<N> Collide<N, Aabr<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn collision(&self, other: &Self) -> Option<Collision<N>> {
        let n = other.position() - self.position();
        let self_extent = self.extent().half();
        let other_extent = other.extent().half();
        let a_w = self_extent.width();
        let b_w = other_extent.width();
        let a_h = self_extent.height();
        let b_h = other_extent.height();

        // Calculate overlap on x axis.
        let x_overlap = a_w + b_w - n.x.abs();

        if x_overlap > N::zero() {
            // Calculate overlap on y axis.
            let y_overlap = a_h + b_h - n.y.abs();

            if y_overlap > N::zero() {
                // We have an overlap on both axes- calculate the area.
                let overlap = self.overlap_unchecked(other);

                // Find out which axis is axis of least penetration
                if x_overlap < y_overlap {
                    let normal = if n.x < N::zero() {
                        Vector::new(-N::one(), N::zero())
                    } else {
                        Vector::new(N::one(), N::zero())
                    };
                    return Some(Collision::new(overlap.position(), normal, x_overlap));
                } else {
                    let normal = if n.y < N::zero() {
                        Vector::new(N::zero(), -N::one())
                    } else {
                        Vector::new(N::zero(), N::one())
                    };
                    return Some(Collision::new(overlap.position(), normal, y_overlap));
                }
            }
        }
        None
    }

    fn enveloping(&self, other: &Self) -> bool {
        let (min_a, max_a) = self.min_max();
        let (min_b, max_b) = other.min_max();
        min_a.x <= min_b.x && max_a.x >= max_b.x && min_a.y <= min_b.y && max_a.y >= max_b.y
    }

    fn enveloped_by(&self, other: &Self) -> bool {
        other.enveloping(self)
    }
}

//=================================================================//
//============================== OBR ==============================//
//=================================================================//

impl<N> Distance<N, Obr<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn distance_squared(&self, obr: &Obr<N>) -> N {
        let a = self.nearest_point(obr);
        let b = obr.nearest_point(&a);
        a.distance_squared(&b)
    }
}

impl<N> Nearest<N, Obr<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn nearest_point(&self, obr: &Obr<N>) -> Point<N> {
        let mut x = self.position().x;
        let mut y = self.position().y;
        let bb = obr.bounding_rect();

        let x_overlapped = if bb.min.x > self.max.x {
            x = bb.min.x;
            false
        } else if bb.max.x < self.min.x {
            x = bb.max.x;
            false
        } else {
            // there is x overlap
            true
        };

        let y_overlapped = if bb.min.y > self.max.y {
            y = bb.min.y;
            false
        } else if bb.max.y < self.min.y {
            y = bb.max.y;
            false
        } else {
            // there is y overlap
            true
        };

        if !x_overlapped && !y_overlapped {
            return Point::new(x, y);
        }

        let temp_point = Point::new(x, y);
        // We need to find the two nearest points on the x axis.
        let mut ordered_verts = obr.vertices();
        ordered_verts.sort_by(|a, b| {
            let dist_a = a.distance_squared(&temp_point);
            let dist_b = b.distance_squared(&temp_point);
            dist_a.partial_cmp(&dist_b).unwrap()
        });
        let edge = Line::new(ordered_verts[0], ordered_verts[1]);
        self.nearest_point(&edge)
    }
}

impl<N> Collide<N, Obr<N>> for Aabr<N>
where
    N: PrimaFloat,
{
    fn collision(&self, _obr: &Obr<N>) -> Option<Collision<N>> {
        todo!()
    }

    fn intersecting(&self, obr: &Obr<N>) -> bool {
        let x_axis = Vector::right();
        let y_axis = Vector::up();
        let (a_x, a_y) = project_shape_to_axis_pair(self, x_axis, y_axis);
        let (b_x, b_y) = project_shape_to_axis_pair(obr, x_axis, y_axis);

        // Compare!
        if !a_x.intersecting(&b_x) || !a_y.intersecting(&b_y) {
            return false;
        }

        let x_axis = obr.x_axis();
        let y_axis = obr.y_axis();

        let (a_x, a_y) = project_shape_to_axis_pair(self, x_axis, y_axis);
        let (b_x, b_y) = project_shape_to_axis_pair(obr, x_axis, y_axis);

        a_x.intersecting(&b_x) && a_y.intersecting(&b_y)
    }

    fn enveloping(&self, obr: &Obr<N>) -> bool {
        obr.vertices().iter().all(|v| self.contains(v))
    }

    fn enveloped_by(&self, obr: &Obr<N>) -> bool {
        self.vertices().iter().all(|v| obr.contains(v))
    }
}
