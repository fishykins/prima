use crate::core::DefaultIx;

pub struct Edge<E, Ix = DefaultIx>
where
    Ix: Clone,
{
    node_a: Ix,
    node_b: Ix,
    cell_a: Ix,
    cell_b: Ix,
    data: E,
}

impl<E, Ix> Edge<E, Ix>
where
    Ix: Clone,
{
    pub fn new(node_a: Ix, node_b: Ix, cell_a: Ix, cell_b: Ix, data: E) -> Self {
        Self {
            node_a,
            node_b,
            cell_a,
            cell_b,
            data,
        }
    }
    pub fn node_a(&self) -> Ix {
        self.node_a.clone()
    }
    pub fn node_b(&self) -> Ix {
        self.node_b.clone()
    }
    pub fn cell_a(&self) -> Ix {
        self.cell_a.clone()
    }
    pub fn cell_b(&self) -> Ix {
        self.cell_b.clone()
    }
    pub fn data(&self) -> &E {
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut E {
        &mut self.data
    }
}
