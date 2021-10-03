use crate::core::{DefaultIx, IndexType};

pub struct Cell<C, Ix = DefaultIx>
where
    Ix: IndexType,
{
    pub(crate) edges: Vec<Ix>,
    pub(crate) data: Option<C>,
}

impl<C, Ix> Cell<C, Ix>
where
    Ix: IndexType,
{
    pub fn new(edges: Vec<Ix>, data: Option<C>) -> Self {
        Self { edges, data }
    }

    pub fn data(&self) -> Option<&C> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_ref()
    }
    pub fn data_mut(&mut self) -> Option<&mut C> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_mut()
    }

    pub fn edges(&self) -> Vec<Ix> {
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
