use super::{EdgeIndex, GraphData};
use crate::core::{DefaultIx, IndexType};
pub struct Node<D, Ix = DefaultIx>
where
    Ix: IndexType,
{
    pub(crate) edges: Vec<EdgeIndex<Ix>>,
    pub data: Option<Box<D>>,
}

impl<D, Ix> Node<D, Ix> where Ix: IndexType {
    pub fn new(edges: Vec<EdgeIndex<Ix>>, data: Option<Box<D>>) -> Self {
        Self { edges, data }
    }
    pub fn edges(&self) -> Vec<EdgeIndex<Ix>> {
        self.edges.clone()
    }
}

impl<D, Ix> GraphData<D> for Node<D, Ix> where Ix: IndexType {
    fn data(&self) -> Option<&Box<D>> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_ref()
    }

    fn data_mut(&mut self) -> Option<&mut Box<D>> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_mut()
    }
}
