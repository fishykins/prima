use crate::core::{DefaultIx, IndexType};
use super::{CellIndex, GraphData, NodeIndex};

pub struct Edge<E, Ix = DefaultIx>
where
    Ix: IndexType,
{
    pub(crate) node_a: NodeIndex<Ix>,
    pub(crate) node_b: NodeIndex<Ix>,
    pub(crate) cell_a: CellIndex<Ix>,
    pub(crate) cell_b: CellIndex<Ix>,
    pub data: Option<Box<E>>,
}

impl<E, Ix> Edge<E, Ix>
where
    Ix: IndexType,
{
    pub fn new(node_a: NodeIndex<Ix>, node_b: NodeIndex<Ix>, cell_a: CellIndex<Ix>, cell_b: CellIndex<Ix>, data: Option<Box<E>>) -> Self {
        Self {
            node_a,
            node_b,
            cell_a,
            cell_b,
            data,
        }
    }
    pub fn nodes(&self) -> (NodeIndex<Ix>, NodeIndex<Ix>) {
        (self.node_a, self.node_b)
    }
    pub fn node_other(&self, i: NodeIndex<Ix>) -> Option<NodeIndex<Ix>> {
        if i == self.node_a {
            return Some(self.node_b);
        } else if i == self.node_b {
            return Some(self.node_a);
        } else {
            None
        }
    }
    pub fn cells(&self) -> (CellIndex<Ix>, CellIndex<Ix>) {
        (self.cell_a, self.cell_b)
    }
    pub fn cell_other(&self, i: CellIndex<Ix>) -> Option<CellIndex<Ix>> {
        if i == self.cell_a {
            return Some(self.cell_b);
        } else if i == self.cell_b {
            return Some(self.cell_a);
        } else {
            None
        }
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

impl<E, Ix> GraphData<E> for Edge<E, Ix>
where
    Ix: IndexType,
{
    fn data(&self) -> Option<&Box<E>> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_ref()
    }

    fn data_mut(&mut self) -> Option<&mut Box<E>> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_mut()
    }
}
