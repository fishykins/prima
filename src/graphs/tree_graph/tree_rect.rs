use crate::core::OrdNum;
use crate::geom::{Rect, Transverse};
use num::Float;

use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct EdgeRef(pub usize, pub Transverse);

#[derive(Debug, Clone, PartialEq)]
pub struct TreeRect<T>
where
    T: OrdNum,
{
    pub(crate) rect: Rect<T, T>,
    pub(crate) parent: Option<usize>,
    pub(crate) children: Vec<usize>,
    pub(crate) edges: Vec<EdgeRef>,
}

impl<T> TreeRect<T>
where
    T: OrdNum + Float,
{
    pub(crate) fn new(rect: Rect<T, T>, parent: Option<usize>) -> Self {
        Self {
            rect,
            parent,
            children: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub(crate) fn active(&self) -> bool {
        self.children.len() == 0
    }

    pub fn rect(&self) -> Rect<T, T> {
        self.rect.clone()
    }

    pub fn edges(&self) -> Vec<EdgeRef> {
        return self.edges.clone()
    }
}
