mod cell;
mod edge;
mod node;
pub mod tree_map;

use crate::core::OrdNum;
use crate::core::{DefaultIx, IndexType};
pub use cell::Cell;
pub use edge::Edge;
pub use node::Node;

#[derive(Copy, Clone)]
pub struct CellRef<'a, C, Ix = DefaultIx>(Ix, &'a Cell<C, Ix>)
where
    Ix: IndexType;

#[derive(Copy, Clone)]
pub struct EdgeRef<'a, E, Ix = DefaultIx>(Ix, &'a Edge<E, Ix>)
where
    Ix: IndexType;

#[derive(Copy, Clone)]
pub struct NodeRef<'a, T, N, Ix = DefaultIx>(Ix, &'a Node<T, N, Ix>)
where
    T: OrdNum,
    Ix: IndexType;

impl<'a, C, Ix> CellRef<'a, C, Ix>
where
    Ix: IndexType,
{
    pub fn new(index: Ix, cell: &'a Cell<C, Ix>) -> Self {
        Self(index, cell)
    }
}

impl<'a, E, Ix> EdgeRef<'a, E, Ix>
where
    Ix: IndexType,
{
    pub fn new(index: Ix, edge: &'a Edge<E, Ix>) -> Self {
        Self(index, edge)
    }
}

impl<'a, T, N, Ix> NodeRef<'a, T, N, Ix>
where
    T: OrdNum,
    Ix: IndexType,
{
    pub fn new(index: Ix, node: &'a Node<T, N, Ix>) -> Self {
        Self(index, node)
    }
}

pub trait Graph<T, C, E, N, Ix = DefaultIx>
where
    T: OrdNum,
    Ix: IndexType,
{
    // Simple getters
    fn cell(&self, index: Ix) -> Option<&Cell<C, Ix>>;
    fn edge(&self, index: Ix) -> Option<&Edge<E, Ix>>;
    fn node(&self, index: Ix) -> Option<&Node<T, N, Ix>>;

    fn cell_mut(&mut self, index: Ix) -> Option<&mut Cell<C, Ix>>;
    fn edge_mut(&mut self, index: Ix) -> Option<&mut Edge<E, Ix>>;
    fn node_mut(&mut self, index: Ix) -> Option<&mut Node<T, N, Ix>>;

    // Getters for Ref handles, which are more versatile when linking
    fn cell_ref(&self, index: Ix) -> Option<CellRef<C, Ix>>;
    fn edge_ref(&self, index: Ix) -> Option<EdgeRef<E, Ix>>;
    fn node_ref(&self, index: Ix) -> Option<NodeRef<T, N, Ix>>;

    // Extract simple item from a handle
    fn cell_from_ref(&self, cell_ref: CellRef<C, Ix>) -> &Cell<C, Ix>;
    fn edge_from_ref(&self, edge_ref: EdgeRef<E, Ix>) -> &Edge<E, Ix>;
    fn node_from_ref(&self, node_ref: NodeRef<T, N, Ix>) -> &Node<T, N, Ix>;

    // Getters for collections
    fn cells(&self) -> Vec<&Cell<C, Ix>>;
    fn edges(&self) -> Vec<&Edge<E, Ix>>;
    fn nodes(&self) -> Vec<&Node<T, N, Ix>>;

    // Relational functions, dealing mainly in handles
    fn cell_edges(&self, cell_ref: CellRef<C, Ix>) -> Vec<EdgeRef<E, Ix>>;
    fn cell_nodes(&self, cell_ref: CellRef<C, Ix>) -> Vec<NodeRef<T, N, Ix>>;
    fn cell_neighbors(&self, cell_ref: CellRef<C, Ix>) -> Vec<CellRef<C, Ix>>;

    fn edge_cells(&self, edge_ref: EdgeRef<E, Ix>) -> (CellRef<C, Ix>, CellRef<C, Ix>);
    fn edge_nodes(&self, edge_ref: EdgeRef<E, Ix>) -> (NodeRef<T, N, Ix>, NodeRef<T, N, Ix>);
    fn edge_neighbors(&self, edge_ref: EdgeRef<E, Ix>) -> Vec<EdgeRef<E, Ix>>;

    fn node_edges(&self, node_ref: NodeRef<T, N, Ix>) -> Vec<EdgeRef<E, Ix>>;
    fn node_cells(&self, node_ref: NodeRef<T, N, Ix>) -> Vec<CellRef<C, Ix>>;
    fn node_neighbors(&self, node_ref: NodeRef<T, N, Ix>) -> Vec<NodeRef<T, N, Ix>>;
}
