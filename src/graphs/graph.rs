use super::{Cell, CellIndex, Edge, EdgeIndex, Node, NodeIndex};
use crate::core::{DefaultIx, IndexType};

/// A basic implimentation of a nodegraph, which consists of nodes, and edges that connect them.
/// Each node and edge can have assosiated data, and a node can be allocated positional data.
pub trait NodeGraph<N, E, Ix = DefaultIx>
where
    Ix: IndexType,
{
    fn node(&self, i: NodeIndex<Ix>) -> Option<Node<N>>;
    fn node_mut(&mut self, i: NodeIndex<Ix>) -> Option<&mut Node<N>>;
    fn edge(&self, i: EdgeIndex<Ix>) -> Option<&Edge<E>>;
    fn edge_mut(&mut self, i: EdgeIndex<Ix>) -> Option<&mut Edge<E>>;

    fn nodes(&self) -> Vec<&Node<N>>;
    fn nodes_mut(&self) -> Vec<&mut Node<N>>;
    fn edges(&self) -> Vec<&Edge<E>>;
    fn edges_mut(&self) -> Vec<&mut Edge<E>>;
}

/// A more advanced implimentation of a nodegraph. Cells are polygons defined by their edges and nodes, 
/// which is useful when representing geometrically based graphs with volume.
pub trait CellGraph<C, N, E, Ix>: NodeGraph<N, E, Ix>
where
    Ix: IndexType,
{
    fn cell(&self, i: CellIndex<Ix>) -> Option<&Cell<C>>;
    fn cell_mut(&mut self, i: CellIndex<Ix>) -> Option<&mut Cell<C>>;

    fn cells(&self) -> Vec<&Cell<C>>;
    fn cells_mut(&self) -> Vec<&mut Cell<C>>;
}
