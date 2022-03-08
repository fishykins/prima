/// A Rectangle in 2D space. Alternative to Aabr. 
#[derive(Debug, Clone, Default)]
pub struct Rect<T> {
    /// The minimum x point of the rectangle.
    pub x: T,
    /// The minimum y point of the rectangle.
    pub y: T,
    /// The width of the rectangle.
    pub w: T,
    /// The height of the rectangle.
    pub h: T,
}