use crate::core::{DefaultIx, IndexType};
use std::collections::HashMap;

use super::{Cell, CellIndex, Edge, EdgeIndex, Node, NodeIndex};

/// A graph consisting of cells, edges and nodes. Nodes have positional data,
/// and the position of cells and edges can be derived from that.
#[derive(Debug, Clone)]
pub struct VectorGraph<D, Ix = DefaultIx>
where
    Ix: IndexType,
{
    /// Collection of nodes
    pub nodes: HashMap<NodeIndex<Ix>, Node<D, Ix>>,
    /// Collection of cells
    pub cells: HashMap<CellIndex<Ix>, Cell<D, Ix>>,
    /// Collection of edges
    pub edges: HashMap<EdgeIndex<Ix>, Edge<D, Ix>>,
}

impl<D, Ix> VectorGraph<D, Ix>
where
    Ix: IndexType,
{
    /// Retrurns an empty VectorGraph.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            cells: HashMap::new(),
        }
    }

    /// Adds a cell to the graph and returns it's index.
    pub fn add_cell(&mut self, cell: Cell<D, Ix>) -> CellIndex<Ix> {
        let index = CellIndex::new(self.cells.len());
        self.cells.insert(index, cell);
        index
    }

    /// Adds a node to the graph and returns it's index.
    /// If already in the graph, returns the index of the existing node and updates.
    pub fn add_node(&mut self, node: Node<D, Ix>) -> NodeIndex<Ix> {
        for  (i, n) in self.nodes.iter_mut() {
            if n.position == node.position {
                // Update the node
                for e in node.edges.iter() {
                    if !n.edges.contains(e) {
                        n.edges.push(e.clone());
                    }
                }
                if n.data.is_none() && node.data.is_some() {
                    n.data = node.data;
                }
                return i.clone();
            }
        }
        let index = NodeIndex::new(self.nodes.len());
        self.nodes.insert(index, node);
        index
    }

    /// Adds an edge to the graph and returns it's index.
    pub fn add_edge(&mut self, edge: Edge<D, Ix>) -> EdgeIndex<Ix> {
        for (i, e) in self.edges.iter() {
           if e == &edge {
               return i.clone();
           }
        }
        let index = EdgeIndex::new(self.edges.len());
        self.edges.insert(index, edge);
        index
    }
}
