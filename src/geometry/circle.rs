use crate::Point2;


/// A simple circle, defined by a center and radius.
/// # Examples
///
/// ```
/// let circle = Circle::new(Point2::new(0.0, 0.0), 32.0);
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct Circle<N> {
    /// The middle of the circle.
    pub center: Point2<N>,
    /// Radius of the circle.
    pub radius: N,
}


impl<N> Circle<N> {
    /// Builds a circle from given center and radius.
    pub fn new(center: Point2<N>, radius: N) -> Self {
        Self { center, radius }
    }
}