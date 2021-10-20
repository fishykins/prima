pub mod tree_graph;

use crate::core::OrdNum;
use crate::core::{DefaultIx, IndexType};
pub use cell::Cell;
pub use edge::Edge;
pub use node::Node;
use vek::{LineSegment2, Vec2};

mod cell;
mod edge;
mod node;

macro_rules! index {
    ($index_type:ident) => {
        #[derive(Copy, Clone, Debug, PartialOrd, Ord, Eq, Hash, Default)]
        pub struct $index_type<Ix = DefaultIx>(pub Ix)
        where
            Ix: IndexType;

        impl<Ix> PartialEq for $index_type<Ix>
        where
            Ix: IndexType,
        {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl<Ix> $index_type<Ix> where Ix: IndexType {
            #[inline(always)]
            pub fn new(x: usize) -> Self {
                Self(Ix::new(x))
            }
            #[inline(always)]
            pub fn index(&self) -> usize {
                self.0.index()
            }
        }
    };
}

pub trait GraphData<T> {
    fn data(&self) -> Option<&Box<T>>;
    fn data_mut(&mut self) -> Option<&mut Box<T>>;
}

index!(EdgeIndex);
index!(CellIndex);
index!(NodeIndex);

pub enum GraphIndex<Ix> where Ix : IndexType {
    Cell(CellIndex<Ix>),
    Edge(EdgeIndex<Ix>),
    Node(NodeIndex<Ix>),
}

pub trait Graph<T, C, E, N, Ix = DefaultIx>
where
    T: OrdNum,
    Ix: IndexType,
{
    // Simple getters
    fn cell(&self, index: CellIndex<Ix>) -> Option<&Cell<C, Ix>>;
    fn edge(&self, index: EdgeIndex<Ix>) -> Option<&Edge<E, Ix>>;
    fn node(&self, index: NodeIndex<Ix>) -> Option<&Node<T, N, Ix>>;

    fn cell_mut(&mut self, index: CellIndex<Ix>) -> Option<&mut Cell<C, Ix>>;
    fn edge_mut(&mut self, index: EdgeIndex<Ix>) -> Option<&mut Edge<E, Ix>>;
    fn node_mut(&mut self, index: NodeIndex<Ix>) -> Option<&mut Node<T, N, Ix>>;

    // Getters for collections
    fn cells(&self) -> Vec<&Cell<C, Ix>>;
    fn edges(&self) -> Vec<&Edge<E, Ix>>;
    fn nodes(&self) -> Vec<&Node<T, N, Ix>>;

    // Relational functions, dealing mainly in handles
    fn cell_edges(&self, cell_ref: CellIndex<Ix>) -> Vec<EdgeIndex<Ix>>;
    fn cell_nodes(&self, cell_ref: CellIndex<Ix>) -> Vec<NodeIndex<Ix>>;
    fn cell_neighbors(&self, cell_ref: CellIndex<Ix>) -> Vec<CellIndex<Ix>>;

    fn edge_cells(&self, edge_ref: EdgeIndex<Ix>) -> (CellIndex<Ix>, CellIndex<Ix>);
    fn edge_nodes(&self, edge_ref: EdgeIndex<Ix>) -> (NodeIndex<Ix>, NodeIndex<Ix>);
    fn edge_neighbors(&self, edge_ref: EdgeIndex<Ix>) -> Vec<EdgeIndex<Ix>>;

    fn node_edges(&self, node_ref: NodeIndex<Ix>) -> Vec<EdgeIndex<Ix>>;
    fn node_cells(&self, node_ref: NodeIndex<Ix>) -> Vec<CellIndex<Ix>>;
    fn node_neighbors(&self, node_ref: NodeIndex<Ix>) -> Vec<NodeIndex<Ix>>;

    // Cute helper classes
    fn line(&self, edge_ref: EdgeIndex<Ix>) -> LineSegment2<T>;
    fn center(&self, cell_ref: CellIndex<Ix>) -> Vec2<T>;
}
