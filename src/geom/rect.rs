use super::{Float, Vec2};
use crate::core::Axis;

/// Axis aligned rectangle
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}

impl Rect {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }.validate()
    }

    pub fn valid(&self) -> bool {
        self.min.x < self.max.x && self.min.y < self.max.y
    }

    pub fn validate(self) -> Self {
        Self {
            min: self.min.min(self.max),
            max: self.min.max(self.max),
        }
    }

    pub fn width(&self) -> Float {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> Float {
        self.max.y - self.min.y
    }

    pub fn center(&self) -> Vec2 {
        (self.min + self.max) / 2.0
    }

    pub fn contains_point(&self, p: Vec2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    pub fn contains_rect(&self, other: &Rect) -> bool {
        other.min.x >= self.min.x
            && other.min.x <= self.max.x
            && other.max.x >= self.min.x
            && other.max.x <= self.max.x
            && other.min.y >= self.min.y
            && other.min.y <= self.max.y
            && other.max.y >= self.min.y
            && other.max.y <= self.max.y
    }

    pub fn collides_with_rect(&self, other: &Rect) -> bool {
        let a = Vec2::new(other.min.x, other.min.y);
        let b = Vec2::new(other.min.x, other.max.y);
        let c = Vec2::new(other.max.x, other.min.y);
        let d = Vec2::new(other.max.x, other.max.y);
        self.contains_point(a)
            || self.contains_point(b)
            || self.contains_point(c)
            || self.contains_point(d)
    }

    pub fn split(self, position: Float, axis: Axis) -> (Self, Self) {
        let p = position.clamp(0.0, 1.0);
        match axis {
            Axis::Vertical => self.split_x(p),
            Axis::Horizontal => self.split_y(p),
            Axis::Both => todo!(),
            Axis::None => panic!("Cannot split by axis None!"),
        }
    }

    pub fn split_x(self, position: Float) -> (Self, Self) {
        let a = Rect::new(
            self.min,
            self.min + Vec2::new(self.width() * position, self.height()),
        );
        let b = Rect::new(self.min + Vec2::new(self.width(), 0.0), self.max);
        (a, b)
    }

    pub fn split_y(self, position: Float) -> (Self, Self) {
        let a = Rect::new(
            self.min,
            self.min + Vec2::new(self.width(), self.height() * position),
        );
        let b = Rect::new(self.min + Vec2::new(0.0, self.height()), self.max);
        (a, b)
    }
}
