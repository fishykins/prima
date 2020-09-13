use vek::Vec3;
use num::{Num, Signed};

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
    None,
}

impl Direction {
    pub fn next(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Front,
            Direction::Front => Direction::Back,
            Direction::Back => Direction::None,
            _ => Direction::None,
        }
    }

    pub fn invert(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Front => Direction::Back,
            Direction::Back => Direction::Front,
            _ => Direction::None,
        }
    }

    pub fn iter() -> Vec<Direction> {
        vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::Front,
            Direction::Back,
        ]
    }

    pub fn to_vec<T: Num + Signed>(&self) -> Vec3<T> {
        match self {
            Direction::Up => Vec3::new(T::zero(), T::one(), T::zero()),
            Direction::Down => Vec3::new(T::zero(), - T::one(), T::zero()),
            Direction::Left => Vec3::new(- T::one(), T::zero(), T::zero()),
            Direction::Right => Vec3::new(T::one(), T::zero(), T::zero()),
            Direction::Front => Vec3::new(T::zero(), T::zero(), T::one()),
            Direction::Back => Vec3::new(T::zero(), T::zero(), - T::one()),
            _ => Vec3::new(T::zero(), T::zero(), T::zero()),
        }
    }

    pub fn is_none(&self) -> bool {
        self.to_vec::<i32>() == Vec3::<i32>::zero()
    }
}

