use super::{EdgeIndex, GraphData};
use crate::core::{DefaultIx, IndexType};

pub struct Cell<C, Ix = DefaultIx>
where
    Ix: IndexType,
{
    pub(crate) edges: Vec<EdgeIndex<Ix>>,
    pub data: Option<Box<C>>,
}

impl<C, Ix> Cell<C, Ix>
where
    Ix: IndexType,
{
    pub fn new(edges: Vec<EdgeIndex<Ix>>, data: Option<Box<C>>) -> Self {
        Self { edges, data }
    }

    pub fn edges(&self) -> Vec<EdgeIndex<Ix>> {
        self.edges.clone()
    }
}

impl<C, Ix> PartialEq for Cell<C, Ix>
where
    Ix: IndexType,
{
    fn eq(&self, other: &Self) -> bool {
        self.edges == other.edges
    }
}

impl<C, Ix> GraphData<C> for Cell<C, Ix>
where
    Ix: IndexType,
{
    fn data(&self) -> Option<&Box<C>> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_ref()
    }

    fn data_mut(&mut self) -> Option<&mut Box<C>> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_mut()
    }
}
