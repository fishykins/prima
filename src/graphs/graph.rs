use super::{Cell, CellIndex, Edge, EdgeIndex, Node, NodeIndex};
use crate::core::{DefaultIx, IndexType};

/// A basic implimentation of a nodegraph, which consists of nodes, and edges that connect them.
/// Each node and edge can have assosiated data, and a node can be allocated positional data.
pub trait NodeGraph<N, E, Ix = DefaultIx>
where
    Ix: IndexType,
{
    /// Returns the node at given index, or [`None`] if not found.
    fn node(&self, i: NodeIndex<Ix>) -> Option<Node<N>>;
    /// Returns the mutable node at given index, or [`None`] if not found.
    fn node_mut(&mut self, i: NodeIndex<Ix>) -> Option<&mut Node<N>>;
    /// Returns the edge at given index, or [`None`] if not found.
    fn edge(&self, i: EdgeIndex<Ix>) -> Option<&Edge<E>>;
    /// Returns the mutable edge at given index, or [`None`] if not found.
    fn edge_mut(&mut self, i: EdgeIndex<Ix>) -> Option<&mut Edge<E>>;

    /// Returns array of assosiated nodes.
    fn nodes(&self) -> Vec<&Node<N>>;
    /// Returns mutable array of assosiated nodes.
    fn nodes_mut(&self) -> Vec<&mut Node<N>>;
    /// Returns array of assosiated edges.
    fn edges(&self) -> Vec<&Edge<E>>;
    /// Returns mutable array of assosiated edges.
    fn edges_mut(&self) -> Vec<&mut Edge<E>>;
}

/// A more advanced implimentation of a nodegraph. Cells are polygons defined by their edges and nodes, 
/// which is useful when representing geometrically based graphs with volume.
pub trait CellGraph<C, N, E, Ix>: NodeGraph<N, E, Ix>
where
    Ix: IndexType,
{
    /// Returns the cell at given index, or [`None`] if not found.
    fn cell(&self, i: CellIndex<Ix>) -> Option<&Cell<C>>;
    /// Returns the mutable cell at given index, or [`None`] if not found.
    fn cell_mut(&mut self, i: CellIndex<Ix>) -> Option<&mut Cell<C>>;
    /// Returns array of assosiated cells.
    fn cells(&self) -> Vec<&Cell<C>>;
    /// Returns mutable array of assosiated cells.
    fn cells_mut(&self) -> Vec<&mut Cell<C>>;
}
