use crate::core::{DefaultIx, IndexType, OrdNum};
use crate::graphs::*;
use vek::Rect;

macro_rules! get_graph_item {
    ($func_name:ident, $func_mut:ident, $func_ref:ident, $func_from_ref:ident, $func_collection:ident, $array:ident, $ref_type:ty, $node_type:ty) => {
        fn $func_name(&self, index: Ix) -> Option<&$node_type> {
            if index.index() >= self.$array.len() {
                return None;
            }
            Some(&self.$array[index.index()])
        }
        fn $func_mut(&mut self, index: Ix) -> Option<&mut $node_type> {
            if index.index() >= self.$array.len() {
                return None;
            }
            Some(&mut self.$array[index.index()])
        }
        fn $func_ref(&self, index: Ix) -> Option<$ref_type> {
            if index.index() >= self.$array.len() {
                return None;
            }
            return Some(<$ref_type>::new(index, &self.$array[index.index()]));
        }
        fn $func_from_ref(&self, node_ref: $ref_type) -> &$node_type {
            &self.$array[node_ref.0.index()]
        }
        fn $func_collection(&self) -> Vec<&$node_type> {
            self.$array.iter().map(|x| x).collect()
        }
    };
}

// A graph for navigating a treemap. As the graph is consructed by a dedicated builder, some assumptions are made about certian calls.
// It is assumed that if a node/cell/edge is in the graph, all its assosiated pointers will be valid.
// This means the only getters that need to check index validity are the main getters from raw indexs.
pub struct TreeGraph<T, C, E, N, Ix = DefaultIx>
where
    T: OrdNum,
    Ix: IndexType,
{
    pub(crate) cells: Vec<Cell<C, Ix>>,
    pub(crate) edges: Vec<Edge<E, Ix>>,
    pub(crate) nodes: Vec<Node<T, N, Ix>>,
    pub(crate) rects: Vec<Rect<T, T>>,
}

// Graph trait
impl<T, C, E, N, Ix> Graph<T, C, E, N, Ix> for TreeGraph<T, C, E, N, Ix>
where
    C: Copy,
    E: Copy,
    N: Copy,
    T: OrdNum,
    Ix: IndexType,
{
    get_graph_item!(cell, cell_mut, cell_ref, cell_from_ref, cells, cells, CellRef<C, Ix>, Cell<C, Ix>);
    get_graph_item!(edge, edge_mut, edge_ref, edge_from_ref, edges, edges, EdgeRef<E, Ix>, Edge<E, Ix>);
    get_graph_item!(node, node_mut, node_ref, node_from_ref, nodes, nodes, NodeRef<T, N, Ix>, Node<T, N, Ix>);

    fn cell_edges(&self, cell_ref: CellRef<C, Ix>) -> Vec<EdgeRef<E, Ix>> {
        let cell = self.cell_from_ref(cell_ref);
        cell.edges()
            .iter()
            .map(|e| self.edge_ref(*e).unwrap())
            .collect()
    }

    // TODO: This is not very efficient, needs fewer loops
    fn cell_nodes(&self, cell_ref: CellRef<C, Ix>) -> Vec<NodeRef<T, N, Ix>> {
        let cell_edges = self.cell_edges(cell_ref);
        let mut nodes = Vec::<NodeRef<T, N, Ix>>::new();

        for e in cell_edges.iter() {
            let edge_nodes = self.edge_nodes(*e);
            let mut found_a = false;
            let mut found_b = false;
            for node in nodes.iter() {
                if node.0 == edge_nodes.0 .0 {
                    found_a = true;
                } else if node.0 == edge_nodes.1 .0 {
                    found_b = true;
                };

                if found_a && found_b {
                    break;
                }
            }
            if !found_a {
                nodes.push(edge_nodes.0);
            }
            if !found_b {
                nodes.push(edge_nodes.1);
            }
        }
        return nodes;
    }

    fn cell_neighbors(&self, cell_ref: CellRef<C, Ix>) -> Vec<CellRef<C, Ix>> {
        let cell_edges = self.cell_edges(cell_ref);
        let mut neighbors = Vec::new();
        for e in cell_edges.iter() {
            let edge = self.edge_from_ref(*e);
            let other = edge.cell_other(cell_ref.0).unwrap();
            neighbors.push(self.cell_ref(other).unwrap());
        }
        return neighbors;
    }

    fn edge_cells(&self, edge_ref: EdgeRef<E, Ix>) -> (CellRef<C, Ix>, CellRef<C, Ix>) {
        let edge = self.edge_from_ref(edge_ref);
        (
            CellRef::new(edge.cell_a, &self.cells[edge.cell_a.index()]),
            CellRef::new(edge.cell_b, &self.cells[edge.cell_b.index()]),
        )
    }

    fn edge_nodes(&self, edge_ref: EdgeRef<E, Ix>) -> (NodeRef<T, N, Ix>, NodeRef<T, N, Ix>) {
        let edge = self.edge_from_ref(edge_ref);
        (
            NodeRef::new(edge.node_a, &self.nodes[edge.node_a.index()]),
            NodeRef::new(edge.node_b, &self.nodes[edge.node_b.index()]),
        )
    }

    fn edge_neighbors(&self, edge_ref: EdgeRef<E, Ix>) -> Vec<EdgeRef<E, Ix>> {
        let (node_a, node_b) = self.edge_nodes(edge_ref);
        let mut neighbors = Vec::new();
        for e in self.node_edges(node_a).iter() {
            if e.0 != edge_ref.0 {
                neighbors.push(*e);
            }
        }
        for e in self.node_edges(node_b).iter() {
            if e.0 != edge_ref.0 {
                neighbors.push(*e);
            }
        }
        return neighbors;
    }

    fn node_edges(&self, node_ref: NodeRef<T, N, Ix>) -> Vec<EdgeRef<E, Ix>> {
        let node = self.node_from_ref(node_ref);
        return node
            .linked_edges()
            .iter()
            .map(|e| self.edge_ref(*e).unwrap())
            .collect();
    }

    fn node_cells(&self, node_ref: NodeRef<T, N, Ix>) -> Vec<CellRef<C, Ix>> {
        let node_edges = self.node_edges(node_ref);
        let mut cells = Vec::<CellRef<C, Ix>>::new();
        for e in node_edges.iter() {
            let (a, b) = self.edge_cells(*e);
            let mut has_a = false;
            let mut has_b = false;
            for c in cells.iter() {
                if c.0 == a.0 {
                    has_a = true;
                } else if c.0 == b.0 {
                    has_b = true;
                }
                if has_a && has_b {
                    break;
                }
            }
            if !has_a {
                cells.push(a);
            }
            if !has_b {
                cells.push(b);
            }
        }
        return cells;
    }

    fn node_neighbors(&self, node_ref: NodeRef<T, N, Ix>) -> Vec<NodeRef<T, N, Ix>> {
        let node_edges = self.node_edges(node_ref);
        let mut neighbors = Vec::new();
        for e in node_edges.iter() {
            let edge = self.edge_from_ref(*e);
            let other = edge.node_other(node_ref.0).unwrap();
            neighbors.push(self.node_ref(other).unwrap());
        }
        return neighbors;
    }
}

// Special functions!
impl<T, C, E, N, Ix> TreeGraph<T, C, E, N, Ix>
where
    T: OrdNum,
    Ix: IndexType,
{
    pub fn cell_rect(&self, cell: CellRef<C, Ix>) -> &Rect<T, T> {
        &self.rects[cell.0.index()]
    }
}

#[cfg(test)]
mod tests {
    use crate::geom::Axis;
    use crate::graphs::{tree_map::*, Graph};
    use vek::Rect;

    fn build_test_graph() -> TreeGraph<f32, u8, u8, u8> {
        let mut builder = TreemapBuilder::<f32>::new(Rect::new(0., 0., 510., 510.));
        builder.intersect_point(0, Axis::Horizontal, 0.25);
        builder.split(1, Axis::Vertical, 2);
        builder.intersect_point(2, Axis::Vertical, 0.75);
        builder.split(6, Axis::Horizontal, 2);
        builder.intersect_point(8, Axis::Vertical, 0.75);
        builder.build_graph()
    }

    #[test]
    fn treegraph_test() {
        let graph = build_test_graph();

        assert_eq!(graph.cells().len(), 8);
        assert_eq!(graph.edges().len(), 25);
    }
}
