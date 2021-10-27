use super::{EdgeIndex, GraphData};
use crate::core::{DefaultIx, IndexType};

/// A Cell is an entry point for data within the graph structure. It is formed by surrounding ['edge']s and represents area data.
pub struct Cell<D, Ix = DefaultIx>
where
    Ix: IndexType,
{
    pub(crate) edges: Vec<EdgeIndex<Ix>>,
    pub data: Option<Box<D>>,
}

impl<D, Ix> Cell<D, Ix>
where
    Ix: IndexType,
{
    pub fn new(edges: Vec<EdgeIndex<Ix>>, data: Option<Box<D>>) -> Self {
        Self { edges, data }
    }

    /// Returns a vec of assosiated edge indicies.
    pub fn edges(&self) -> Vec<EdgeIndex<Ix>> {
        self.edges.clone()
    }
}

impl<D, Ix> PartialEq for Cell<D, Ix>
where
    Ix: IndexType,
{
    fn eq(&self, other: &Self) -> bool {
        self.edges == other.edges
    }
}

impl<D, Ix> GraphData<D> for Cell<D, Ix>
where
    Ix: IndexType,
{
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
