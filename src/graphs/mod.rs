mod cell;
mod edge;
mod node;
pub mod tree_map;

use crate::core::OrdNum;
pub use cell::Cell;
pub use edge::Edge;
pub use node::Node;

use crate::core::DefaultIx;

pub trait Graph<C, E, N, T, Ix = DefaultIx>
where
    T: OrdNum,
    Ix: Clone,
{
    fn cell(&self, index: Ix) -> Cell<C, Ix>;
    fn edge(&self, index: Ix) -> Edge<E, Ix>;
    fn node(&self, index: Ix) -> Node<T, N, Ix>;

    fn cell_index(&self, cell: &Cell<C, Ix>) -> Ix;
    fn edge_index(&self, edge: &Edge<E, Ix>) -> Ix;
    fn node_index(&self, node: &Node<T, N, Ix>) -> Ix;

    fn cells(&self, index: Ix) -> Vec<&Cell<C, Ix>>;
    fn edges(&self, index: Ix) -> Vec<&Edge<E, Ix>>;
    fn nodes(&self, index: Ix) -> Vec<&Node<T, N, Ix>>;

    fn cell_edges(&self, cell: &Cell<C, Ix>) -> Vec<&Edge<E, Ix>>;
    fn cell_nodes(&self, cell: &Cell<C, Ix>) -> Vec<&Node<T, N, Ix>>;
    fn cell_neighbors(&self, cell: &Cell<C, Ix>) -> Vec<&Cell<C, Ix>>;

    fn edge_cells(&self, edge: &Edge<E, Ix>) -> (Cell<C, Ix>, Cell<C, Ix>);
    fn edge_nodes(&self, edge: &Edge<E, Ix>) -> (&Node<T, N, Ix>, &Node<T, N, Ix>);
    fn edge_neighbors(&self, edge: &Edge<E, Ix>) -> Vec<&Edge<E, Ix>>;

    fn node_edges(&self, node: &Node<T, N, Ix>) -> Vec<&Edge<E, Ix>>;
    fn node_cells(&self, node: &Node<T, N, Ix>) -> Vec<&Cell<C, Ix>>;
    fn node_neighbors(&self, node: &Node<T, N, Ix>) -> Vec<&Node<T, N, Ix>>;
}
