use crate::core::{DefaultIx, IndexType};

pub struct Edge<E, Ix = DefaultIx>
where
    Ix: IndexType,
{
    pub(crate) node_a: Ix,
    pub(crate) node_b: Ix,
    pub(crate) cell_a: Ix,
    pub(crate) cell_b: Ix,
    pub(crate) data: Option<E>,
}

impl<E, Ix> Edge<E, Ix>
where
    Ix: IndexType,
{
    pub fn new(node_a: Ix, node_b: Ix, cell_a: Ix, cell_b: Ix, data: Option<E>) -> Self {
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
    pub fn node_other(&self, i: Ix) -> Option<Ix> {
        if i == self.node_a {
            return Some(self.node_b);
        } else if i == self.node_b {
            return Some(self.node_a);
        } else {
            None
        }
    }
    pub fn cell_a(&self) -> Ix {
        self.cell_a.clone()
    }
    pub fn cell_b(&self) -> Ix {
        self.cell_b.clone()
    }
    pub fn cell_other(&self, i: Ix) -> Option<Ix> {
        if i == self.cell_a {
            return Some(self.cell_b);
        } else if i == self.cell_b {
            return Some(self.cell_a);
        } else {
            None
        }
    }
    pub fn data(&self) -> Option<&E> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_ref()
    }
    pub fn data_mut(&mut self) -> Option<&mut E> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_mut()
    }
}

impl<E, Ix> PartialEq for Edge<E, Ix>
where
    Ix: IndexType,
{
    fn eq(&self, other: &Self) -> bool {
        self.node_a == other.node_a &&
        self.node_b == other.node_b &&
        self.cell_a == other.cell_a &&
        self.cell_b == other.cell_b
    }
}