use crate::core::OrdNum;
use super::Line;
use super::is_triangle_convex;
use vek::{Vec2};
use std::cmp::Ordering;

#[derive(PartialEq)]
pub enum Orientation {
    Linear,
    Clockwise,
    CounterClockwise,
}

pub struct Triangle<T> where T: OrdNum {
    pub a: Vec2<T>,
    pub b: Vec2<T>,
    pub c: Vec2<T>,
}

impl<T> Triangle<T> where T: OrdNum {
    pub fn new(a: Vec2<T>, b: Vec2<T>, c: Vec2<T>) -> Self {
        Self {
            a,b,c
        }
    }

    pub fn ab(&self) -> Line<T> {
        Line {
            start: self.a,
            end: self.b,
        }
    }

    pub fn bc(&self) -> Line<T> {
        Line {
            start: self.b,
            end: self.c,
        }
    }

    pub fn ca(&self) -> Line<T> {
        Line {
            start: self.c,
            end: self.a,
        }
    }

    pub fn centroid(&self) -> Vec2<T> {
        let x = self.a.x + self.b.x + self.c.x;
        let y = self.a.y + self.b.y + self.c.y;
        let three = T::one() + T::one() + T::one();
        Vec2::new(x / three, y / three)
    }

    pub fn contains_point(&self, p: Vec2<T>) -> bool {
        let v0x = self.c.x - self.a.x;
        let v0y = self.c.y - self.a.y;
        let v1x = self.b.x - self.a.x;
        let v1y = self.b.y - self.a.y;
        let v2x = p.x - self.a.x;
        let v2y = p.y - self.a.y;

        let dot00 = v0x * v0x + v0y * v0y;
        let dot01 = v0x * v1x + v0y * v1y;
        let dot02 = v0x * v2x + v0y * v2y;
        let dot11 = v1x * v1x + v1y * v1y;
        let dot12 = v1x * v2x + v1y * v2y;

        let denom = dot00 * dot11 - dot01 * dot01;
        let u = (dot11 * dot02 - dot01 * dot12) / denom;
        let v = (dot00 * dot12 - dot01 * dot02) / denom;

        (u >= T::zero()) && (v >= T::zero()) && (u + v < T::one())
    }

    pub fn is_convex(&self) -> bool {
        is_triangle_convex(self.a, self.b, self.c)
    }

    pub fn orientation(&self) -> Orientation {
        let val = (self.b.y - self.a.y) * (self.c.x - self.b.x) - (self.b.x - self.a.x) * (self.c.y - self.b.y);

        match val.partial_cmp(&T::zero()).expect("Cannot get triangle orientation when val = zero") {
            Ordering::Less => Orientation::CounterClockwise,
            Ordering::Greater => Orientation::Clockwise,
            Ordering::Equal => Orientation::Linear,
        }
    }

    pub fn to_tripple(self) -> (Vec2<T>, Vec2<T>, Vec2<T>) {
        (self.a, self.b, self.c)
    }
}