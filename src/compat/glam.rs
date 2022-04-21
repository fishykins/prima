use glam::Vec2;
use crate::Point;

impl Into<Vec2> for Point {
    fn into(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl Into<Point> for Vec2 {
    fn into(self) -> Point {
        Point::new(self.x, self.y)
    }
}