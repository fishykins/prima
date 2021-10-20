use super::{EdgeIndex, GraphData};
use crate::core::{DefaultIx, IndexType, OrdNum};
#[cfg(not(feature = "godot"))]
use vek::Vec2;

pub struct Node<T, N, Ix = DefaultIx>
where
    T: OrdNum,
    Ix: IndexType,
{
    x: T,
    y: T,
    data: Option<Box<N>>,
    pub(crate) linked_edges: Vec<EdgeIndex<Ix>>,
}

impl<T, N, Ix> Node<T, N, Ix>
where
    T: OrdNum,
    Ix: IndexType,
{
    pub fn new(x: T, y: T, linked_edges: Vec<EdgeIndex<Ix>>, data: Option<Box<N>>) -> Self {
        Self {
            x,
            y,
            linked_edges,
            data,
        }
    }

    #[cfg(not(feature = "godot"))]
    pub fn pos(&self) -> Vec2<T> {
        Vec2::new(self.x.clone(), self.y.clone())
    }

    #[cfg(feature = "godot")]
    pub fn pos(&self) -> Vec2<T> {
        Vec2::new(self.x.clone(), self.y.clone())
    }

    pub fn linked_edges(&self) -> Vec<EdgeIndex<Ix>> {
        self.linked_edges.clone()
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

impl<T, N, Ix> GraphData<N> for Node<T, N, Ix>
where
    T: OrdNum,
    Ix: IndexType,
{
    fn data(&self) -> Option<&Box<N>> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_ref()
    }

    fn data_mut(&mut self) -> Option<&mut Box<N>> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_mut()
    }
}
