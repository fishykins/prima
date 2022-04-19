use glam::{Vec2, };
use crate::Point2;

impl Into<Vec2> for Point2 {
    fn into(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl Into<Point2> for Vec2 {
    fn into(self) -> Point2 {
        Point2::new(self.x, self.y)
    }
}