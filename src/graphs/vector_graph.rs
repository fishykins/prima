use crate::{core::{DefaultIx, IndexType}, geom::Line2};
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
    /// If a node is already in the graph at the same position, returns the index of the existing node and updates.
    pub fn add_or_update_node(&mut self, node: Node<D, Ix>) -> NodeIndex<Ix> {
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
    /// When added, the edge is linked to the cells of the nodes it connects automatically.
    pub fn add_edge(&mut self, edge: Edge<D, Ix>) -> EdgeIndex<Ix> {
        for (i, e) in self.edges.iter() {
           if e == &edge {
               return i.clone();
           }
        }
        let index = EdgeIndex::new(self.edges.len());
        self.edges.insert(index, edge);
        // Check if the refferenced nodes and cells know about this edge
        let node_ai = self.edges[&index].node_a;
        let node_bi = self.edges[&index].node_b;
        let cell_ai = self.edges[&index].cell_a;
        let opt_cell_bi = self.edges[&index].cell_b;

        let node_a = self.nodes.get_mut(&node_ai).expect("Node 'a' not found in graph");
        if !node_a.edges.contains(&index) {
            node_a.edges.push(index.clone());
        }
        let node_b = self.nodes.get_mut(&node_bi).expect("Node 'b' not found in graph");
        if !node_b.edges.contains(&index) {
            node_b.edges.push(index.clone());
        }
        let cell_a = self.cells.get_mut(&cell_ai).expect("Cell 'a' not found in graph");
        if !cell_a.edges.contains(&index) {
            cell_a.edges.push(index.clone());
        }
        if let Some(cell_bi) = opt_cell_bi {
            let cell_b = self.cells.get_mut(&cell_bi).expect("Cell 'b' not found in graph");
            if !cell_b.edges.contains(&index) {
                cell_b.edges.push(index.clone());
            }
        }
        index
    }

    /// Builds a [Line2] from the given edge.
    pub fn edge_line(&self, edge: EdgeIndex<Ix>) -> Option<Line2> {
        if !self.edges.contains_key(&edge) {
            return None;
        }
        let point_a  = self.nodes[&self.edges[&edge].node_a].position;
        let point_b  = self.nodes[&self.edges[&edge].node_b].position;
        return Some(Line2::new(point_a, point_b));
    }
}
