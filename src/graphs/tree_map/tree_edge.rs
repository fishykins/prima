use crate::geom::{Axis, Line};
use crate::core::{OrdNum};
use num::{Float};
use std::{fmt::Debug};




#[derive(Debug, Copy, Clone, PartialEq)]
// A graph edge between two points, connecting two TreeRects.
pub struct TreeEdge<T> where T: OrdNum + Float {
    pub(crate) a: usize,
    pub(crate) b: usize,
    pub(crate) line: Line<T>,
    pub(crate) axis: Axis,
    pub(crate) birth_cycle: Option<usize>,
}


impl<T> TreeEdge<T> where T: OrdNum + Float {
    pub(crate) fn active(&self) -> bool {
        self.birth_cycle.is_none()
    }

    pub fn rect_a(&self) -> usize {
        self.a.clone()
    }

    pub fn rect_b(&self) -> usize {
        self.b.clone()
    }

    pub fn axis(&self) -> Axis {
        self.axis.clone()
    }

    pub fn other(&self, index: usize) -> Option<usize> {
        if self.a == self.b {
            // This is a single edge so no neighbor
            return None;
        }
        if self.a != index && self.b != index {
            return None;
        }
        if self.a == index {
            return Some(self.b);
        }
        Some(self.a)
    }

    pub fn line(&self) -> &Line<T> {
        &self.line
    }

    pub fn line_mut(&mut self) -> &mut Line<T> {
        &mut self.line
    }

    pub(crate) fn can_split(&self, current_cycle: usize) -> bool {
        if self.birth_cycle.is_some() {
            self.birth_cycle.unwrap() == current_cycle
        } else {
            true
        }
    }
}