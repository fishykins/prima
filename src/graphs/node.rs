use crate::core::{DefaultIx, IndexType, OrdNum};
use vek::Vec2;

pub struct Node<T, N, Ix = DefaultIx>
where
    T: OrdNum,
    Ix: IndexType,
{
    x: T,
    y: T,
    pub(crate) data: Option<N>,
    pub(crate) linked_edges: Vec<Ix>,
}

impl<T, N, Ix> Node<T, N, Ix>
where
    T: OrdNum,
    Ix: IndexType,
{
    pub fn new(x: T, y: T, linked_edges: Vec<Ix>, data: Option<N>) -> Self {
        Self {
            x,
            y,
            linked_edges,
            data,
        }
    }

    pub fn pos(&self) -> Vec2<T> {
        Vec2::new(self.x.clone(), self.y.clone())
    }

    pub fn linked_edges(&self) -> &Vec<Ix> {
        &self.linked_edges
    }

    pub fn data(&self) -> Option<&N> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_ref()
    }
    pub fn data_mut(&mut self) -> Option<&mut N> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_mut()
    }
}

impl<T, N, Ix> PartialEq for Node<T, N, Ix>
where
    T: OrdNum,
    Ix: IndexType,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.linked_edges == other.linked_edges
    }
}
