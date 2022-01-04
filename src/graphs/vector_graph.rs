use std::marker::PhantomData;

use crate::{
    core::{DefaultIx, IndexType},
    geom::Vec2,
};

use super::{Cell, CellIndex, Edge, EdgeIndex, Node, NodeIndex};

macro_rules! graph_item {
    ($item_type:ty, $index_type:ty, $collection:ident, $getter:ident, $getter_mut:ident, $getter_all:ident, $getter_all_mut:ident, $adder:ident) => {
        /// Getter for graph item.
        pub fn $getter(&self, index: $index_type) -> Option<&$item_type> {
            if index.index() >= self.$collection.len() {
                return None;
            }
            Some(&self.$collection[index.index()])
        }
        /// Mutable getter for graph item.
        pub fn $getter_mut(&mut self, index: $index_type) -> Option<&mut $item_type> {
            if index.index() >= self.$collection.len() {
                return None;
            }
            Some(&mut self.$collection[index.index()])
        }
        /// Getter for item collection.
        pub fn $getter_all(&self) -> Vec<&$item_type> {
            self.$collection.iter().map(|x| x).collect()
        }
        /// Mutable getter for item collection.
        pub fn $getter_all_mut(&mut self) -> Vec<&mut $item_type> {
            self.$collection.iter_mut().map(|x| x).collect()
        }
        /// Adds the given item to the graph.
        pub fn $adder(&mut self, item: $item_type) -> $index_type {
            self.$collection.push(item);
            <$index_type>::new(self.$collection.len() - 1)
        }
    };
}

/// A graph consisting of cells, edges and nodes. Nodes have positional data,
/// and the position of cells and edges can be derived from that.
pub struct VectorGraph<N, E, C, Ix = DefaultIx>
where
    Ix: IndexType,
{
    /// Boilerplate
    _phantom_data: PhantomData<Ix>,
    /// Collection of nodes
    nodes: Vec<Node<N, Ix>>,
    /// Collection of cells
    cells: Vec<Cell<C, Ix>>,
    /// Collection of edges
    edges: Vec<Edge<E, Ix>>,
}

impl<N, E, C, Ix> VectorGraph<N, E, C, Ix>
where
    Ix: IndexType,
{
    /// Retrurns an empty VectorGraph.
    pub fn new() -> Self {
        Self {
            _phantom_data: PhantomData::default(),
            nodes: Vec::new(),
            edges: Vec::new(),
            cells: Vec::new(),
        }
    }

    graph_item!(Cell<C, Ix>, CellIndex<Ix>, cells, cell, cell_mut, cells, cells_mut, add_cell);
    graph_item!(Edge<E, Ix>, EdgeIndex<Ix>, edges, edge, edge_mut, edges, edges_mut, add_edge);
    graph_item!(Node<N, Ix>, NodeIndex<Ix>, nodes, node, node_mut, nodes, nodes_mut, add_node);

    /// Finds the node at given position or returns none.
    pub fn try_get_node(&self, pos: Vec2) -> Option<NodeIndex<Ix>> {
        if let Some(i) = self.nodes.iter().position(|x| x.position == pos) {
            return Some(NodeIndex::new(i));
        }
        None
    }

    /// Finds the node at given position or adds an empty one if none found.
    pub fn get_node(&mut self, pos: Vec2) -> NodeIndex<Ix> {
        if let Some(i) = self.nodes.iter().position(|x| x.position == pos) {
            return NodeIndex::new(i);
        }
        self.add_node(Node::new(pos, Vec::new(), None))
    }

    /// Adds or replaces the given node, based on positional data. 
    pub fn update_node(&mut self, node: Node<N, Ix>) -> NodeIndex<Ix> {
        let i = self.try_get_node(node.position);
        if i.is_some() {
            self.nodes[i.unwrap().index()] = node;
            return i.unwrap();
        }
        self.add_node(node)
    }

    /// Tries to find the edge that connects a and b. Returns none if not found.
    /// If bidirectional, the edge can be 'a -> b' or 'b -> a'.
    pub fn try_get_edge(
        &self,
        a: NodeIndex<Ix>,
        b: NodeIndex<Ix>,
        bidirectional: bool,
    ) -> Option<EdgeIndex<Ix>> {
        let edge = self
            .edges
            .iter()
            .position(|x| x.node_a == a && x.node_b == b);
        if edge.is_some() {
            return Some(EdgeIndex::new(edge.unwrap()));
        }
        if bidirectional {
            let edge = self
                .edges
                .iter()
                .position(|x| x.node_b == a && x.node_a == b);
            if edge.is_some() {
                return Some(EdgeIndex::new(edge.unwrap()));
            }
        }
        None
    }

    /// gets the edge connecting a and b, or adds a new one (using the passed cell index as `cell_a`).
    pub fn get_edge(
        &mut self,
        a: NodeIndex<Ix>,
        b: NodeIndex<Ix>,
        cell: CellIndex<Ix>,
        bidirectional: bool,
    ) -> EdgeIndex<Ix> {
        let edge = self
            .edges
            .iter()
            .position(|x| x.node_a == a && x.node_b == b);
        if edge.is_some() {
            return EdgeIndex::new(edge.unwrap());
        }
        if bidirectional {
            let edge = self
                .edges
                .iter()
                .position(|x| x.node_b == a && x.node_a == b);
            if edge.is_some() {
                return EdgeIndex::new(edge.unwrap());
            }
        }
        self.add_edge(Edge::new_single_cell(a, b, cell, None))
    }

    // TODO: Find a nicer way to index nodes by position and edges by 'a -> b'
}
