use crate::core::{DefaultIx, IndexType, OrdNum};
use crate::graphs::*;
use vek::{LineSegment2, Rect};

// macro_rules! get_graph_item {
//     ($collection:ident, $getter:ident, $getter_mut:ident, $getter_all:ident, $index_type:ty, $node_type:ty) => {
//         fn $getter(&self, index: $index_type) -> Option<&$node_type> {
//             if index.index() >= self.$collection.len() {
//                 return None;
//             }
//             Some(&self.$collection[index.index()])
//         }
//         fn $getter_mut(&mut self, index: $index_type) -> Option<&mut $node_type> {
//             if index.index() >= self.$collection.len() {
//                 return None;
//             }
//             Some(&mut self.$collection[index.index()])
//         }
//         fn $getter_all(&self) -> Vec<&$node_type> {
//             self.$collection.iter().map(|x| x).collect()
//         }
//     };
// }

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
    fn cell(&self, index: CellIndex<Ix>) -> Option<&Cell<C, Ix>> {
        if index.index() >= self.cells.len() {
            return None;
        }
        Some(&self.cells[index.index()])
    }

    fn edge(&self, index: EdgeIndex<Ix>) -> Option<&Edge<E, Ix>> {
        if index.index() >= self.edges.len() {
            return None;
        }
        Some(&self.edges[index.index()])
    }

    fn node(&self, index: NodeIndex<Ix>) -> Option<&Node<T, N, Ix>> {
        if index.index() >= self.cells.len() {
            return None;
        }
        Some(&self.nodes[index.index()])
    }

    fn cell_mut(&mut self, index: CellIndex<Ix>) -> Option<&mut Cell<C, Ix>> {
        if index.index() >= self.cells.len() {
            return None;
        }
        Some(&mut self.cells[index.index()])
    }

    fn edge_mut(&mut self, index: EdgeIndex<Ix>) -> Option<&mut Edge<E, Ix>> {
        if index.index() >= self.edges.len() {
            return None;
        }
        Some(&mut self.edges[index.index()])
    }

    fn node_mut(&mut self, index: NodeIndex<Ix>) -> Option<&mut Node<T, N, Ix>> {
        if index.index() >= self.cells.len() {
            return None;
        }
        Some(&mut self.nodes[index.index()])
    }

    fn cells(&self) -> Vec<&Cell<C, Ix>> {
        self.cells.iter().map(|x| x).collect()
    }

    fn edges(&self) -> Vec<&Edge<E, Ix>> {
        self.edges.iter().map(|x| x).collect()
    }


    fn nodes(&self) -> Vec<&Node<T, N, Ix>> {
        self.nodes.iter().map(|x| x).collect()
    }

    fn cell_edges(&self, cell_ref: CellIndex<Ix>) -> Vec<EdgeIndex<Ix>> {
        let cell = self.cell(cell_ref).expect("Cell index out of bounds");
        return cell.edges()
    }

    // TODO: This is not very efficient, needs fewer loops
    fn cell_nodes(&self, cell_ref: CellIndex<Ix>) -> Vec<NodeIndex<Ix>> {
        let cell_edges = self.cell_edges(cell_ref);
        let mut nodes = Vec::<NodeIndex<Ix>>::new();

        for e in cell_edges.iter() {
            let edge_nodes = self.edge_nodes(*e);
            let mut found_a = false;
            let mut found_b = false;
            for node in nodes.iter() {
                if node.index() == edge_nodes.0.index() {
                    found_a = true;
                } else if node.index() == edge_nodes.1.index() {
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

    fn cell_neighbors(&self, cell_ref: CellIndex<Ix>) -> Vec<CellIndex<Ix>> {
        let cell_edges = self.cell_edges(cell_ref);
        let mut neighbors = Vec::new();
        for e in cell_edges.iter() {
            let edge = self.edge(*e).unwrap();
            let other = edge.cell_other(cell_ref).unwrap();
            neighbors.push(other);
        }
        return neighbors;
    }

    fn edge_cells(&self, edge_index: EdgeIndex<Ix>) -> (CellIndex<Ix>, CellIndex<Ix>) {
        let edge = self.edge(edge_index).expect("Edge index out of bounds");
        edge.cells()
    }

    fn edge_nodes(&self, edge_ref: EdgeIndex<Ix>) -> (NodeIndex<Ix>, NodeIndex<Ix>) {
        let edge = self.edge(edge_ref).expect("Edge index out of bounds");
        edge.nodes()
    }

    fn edge_neighbors(&self, edge_ref: EdgeIndex<Ix>) -> Vec<EdgeIndex<Ix>> {
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

    fn node_edges(&self, node_ref: NodeIndex<Ix>) -> Vec<EdgeIndex<Ix>> {
        let node = self.node(node_ref).expect("Node index out of bounds");
        return node.linked_edges()
    }

    fn node_cells(&self, node_ref: NodeIndex<Ix>) -> Vec<CellIndex<Ix>> {
        let node_edges = self.node_edges(node_ref);
        let mut cells = Vec::<CellIndex<Ix>>::new();
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

    fn node_neighbors(&self, node_ref: NodeIndex<Ix>) -> Vec<NodeIndex<Ix>> {
        let node_edges = self.node_edges(node_ref);
        let mut neighbors = Vec::new();
        for e in node_edges.iter() {
            let edge = self.edge(*e).expect("Edge index out of bounds");
            let other = edge.node_other(node_ref).unwrap();
            neighbors.push(other);
        }
        return neighbors;
    }

    fn line(&self, edge_ref: EdgeIndex<Ix>) -> LineSegment2<T> {
        let (a, b) = self.edge_nodes(edge_ref);
        let node_a = self.node(a).unwrap();
        let node_b = self.node(b).unwrap();
        LineSegment2 {
            start: node_a.pos(),
            end: node_b.pos(),
        }
    }

    fn center(&self, cell_ref: CellIndex<Ix>) -> Vec2<T> {
        let rect = self.cell_rect(cell_ref);
        return rect.center();
    }
}

// Special unique functions!
impl<T, C, E, N, Ix> TreeGraph<T, C, E, N, Ix>
where
    T: OrdNum,
    Ix: IndexType,
{
    pub fn cell_rect(&self, cell: CellIndex<Ix>) -> &Rect<T, T> {
        &self.rects[cell.0.index()]
    }
}

#[cfg(test)]
mod tests {
    use crate::geom::Axis;
    use crate::graphs::{tree_graph::*, Graph};
    use vek::Rect;

    fn build_test_graph() -> TreeGraph<f32, u8, u8, u8> {
        let mut builder = TreeBuilder::<f32>::new(Rect::new(0., 0., 510., 510.));
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
        assert_eq!(graph.nodes().len(), 18);
    }

    #[cfg(feature = "rendering")]
    #[test]
    fn treegraph_render_test() {
        let graph = build_test_graph();
        let mut image = image::RgbImage::new(510, 510);
        crate::render::draw_graph(&mut image, Box::new(&graph), false);
        let _ = image.save("treegraph_render_test.png").unwrap();
    }
}
