use super::{EdgeIndex, GraphData};
use crate::core::{DefaultIx, IndexType};

/// A node represents an anchor point in a graph. It typically has positional data
/// in the form of a [`glam::Vec2`], and can hold aditional data for the end user. 
pub struct Node<D, Ix = DefaultIx>
where
    Ix: IndexType,
{
    pub(crate) edges: Vec<EdgeIndex<Ix>>,
    /// The assosiated data attached to this node.
    pub data: Option<Box<D>>,
}

impl<D, Ix> Node<D, Ix> where Ix: IndexType {
    /// Produces a node with the given edges and optional data.
    pub fn new(edges: Vec<EdgeIndex<Ix>>, data: Option<Box<D>>) -> Self {
        Self { edges, data }
    }
    /// Returns an array of assosiated edges.
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
