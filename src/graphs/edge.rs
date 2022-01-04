use super::{CellIndex, GraphData, NodeIndex};
use crate::core::{DefaultIx, IndexType};

/// An edge that connects two [`super::Node`]s and two [`super::Cell`]s together. Carries data for the end user.
pub struct Edge<E, Ix = DefaultIx>
where
    Ix: IndexType,
{
    pub(crate) node_a: NodeIndex<Ix>,
    pub(crate) node_b: NodeIndex<Ix>,
    pub(crate) cell_a: CellIndex<Ix>,
    // Optional to allow for the initiation of edge edges!
    pub(crate) cell_b: Option<CellIndex<Ix>>,
    /// The assosiated data attached to this Edge.
    pub data: Option<Box<E>>,
}

impl<E, Ix> Edge<E, Ix>
where
    Ix: IndexType,
{
    /// Produces a new Edge with the given nodes, cells and optional data.
    pub fn new(
        node_a: NodeIndex<Ix>,
        node_b: NodeIndex<Ix>,
        cell_a: CellIndex<Ix>,
        cell_b: CellIndex<Ix>,
        data: Option<Box<E>>,
    ) -> Self {
        Self {
            node_a,
            node_b,
            cell_a,
            cell_b: Some(cell_b),
            data,
        }
    }

    /// Creates an edge that is only connected to one cell, rather than the typical two.
    pub fn new_single_cell(
        node_a: NodeIndex<Ix>,
        node_b: NodeIndex<Ix>,
        cell_a: CellIndex<Ix>,
        data: Option<Box<E>>,
    ) -> Self {
        Self {
            node_a,
            node_b,
            cell_a,
            cell_b: None,
            data,
        }
    }

    /// If no second cell is set, adds given cell and returns [`true`], otherwise returns [`false`].
    pub fn try_add_cell(&mut self, i: CellIndex<Ix>) -> bool {
        if self.cell_b.is_none() {
            self.cell_b = Some(i);
            return true;
        }
        false
    }

    /// Returns the two nodes attached to this edge.
    pub fn nodes(&self) -> (NodeIndex<Ix>, NodeIndex<Ix>) {
        (self.node_a, self.node_b)
    }
    /// Given one of two nodes, returns its counterpart. Returns [`None`] if the given node is not found.
    pub fn node_other(&self, i: NodeIndex<Ix>) -> Option<NodeIndex<Ix>> {
        if i == self.node_a {
            return Some(self.node_b);
        } else if i == self.node_b {
            return Some(self.node_a);
        } else {
            None
        }
    }
    /// Returns the two cells that connect to this edge.
    pub fn cells(&self) -> (CellIndex<Ix>, CellIndex<Ix>) {
        (self.cell_a, self.cell_b.expect("msg"))
    }
    /// Given one of two cells, returns its counterpart. Returns [`None`] if the given cell is not found.
    pub fn cell_other(&self, i: CellIndex<Ix>) -> Option<CellIndex<Ix>> {
        if self.cell_b.is_none() {
            return Some(self.cell_a);
        }
        if i == self.cell_a {
            return Some(self.cell_b.unwrap());
        } else if i == self.cell_b.unwrap() {
            return Some(self.cell_a);
        }
        None
    }

    /// Returns true if the given cell index is part of this edge
    pub fn touches_cell(&self, i: CellIndex<Ix>) -> bool {
        if self.cell_b.is_some() {
            return self.cell_b.unwrap() == i || self.cell_a == i;
        }
        self.cell_a == i
    }
}

impl<E, Ix> PartialEq for Edge<E, Ix>
where
    Ix: IndexType,
{
    fn eq(&self, other: &Self) -> bool {
        self.node_a == other.node_a
            && self.node_b == other.node_b
            && self.cell_a == other.cell_a
            && self.cell_b == other.cell_b
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
