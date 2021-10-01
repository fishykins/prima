use crate::core::DefaultIx;

pub struct Cell<C, Ix = DefaultIx>
where
    Ix: Clone,
{
    edges: Vec<Ix>,
    data: C,
}

impl<C, Ix> Cell<C, Ix>
where
    Ix: Clone,
{
    pub fn new(edges: Vec<Ix>, data: C) -> Self {
        Self { edges, data }
    }

    pub fn data(&self) -> &C {
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut C {
        &mut self.data
    }

    pub fn edges(&self) -> Vec<Ix> {
        self.edges.clone()
    }
}
