use crate::{
    core::{Axis, IndexType},
    geom::{Line2, Rect, Vec2},
    graphs::{Cell, CellIndex, Edge, VectorGraph},
};

struct RtCell {
    rect: Rect,
    neighbors: Vec<usize>,
}

/// A graph builder for RectTrees.
pub struct RectTree {
    cells: Vec<RtCell>,
}

impl RectTree {
    /// Returns a new RectTree builder.
    pub fn new(rect: Rect) -> Self {
        Self {
            cells: vec![RtCell {
                rect,
                neighbors: Vec::new(),
            }],
        }
    }

    /// Splits the given rect and returns the index of the new rect (the inital rect is replaced).
    pub fn split(&mut self, i: usize, position: f32, axis: Axis) -> usize {
        let j = self.cells.len();
        let old_cell = &self.cells[i];
        let (a, b) = old_cell.rect.split(position, axis);
        let mut a_neighbors = old_cell.neighbors.clone();
        a_neighbors.push(j);
        let mut b_neighbors = old_cell.neighbors.clone();
        b_neighbors.push(i);
        self.cells[i] = RtCell {
            rect: a,
            neighbors: a_neighbors,
        };
        self.cells.push(RtCell {
            rect: b,
            neighbors: b_neighbors,
        });
        return j;
    }

    /// Splits the given rect into n equal rects along the given axis.
    pub fn split_n(&mut self, i: usize, n: usize, axis: Axis) {
        let cell = &self.cells[i];
        let x = cell.rect.min.x;
        let y = cell.rect.min.y;
        let (w, h, p, q) = match axis {
            Axis::Horizontal => (cell.rect.width(), cell.rect.height()/n as f32, 0.0, 1.0),
            Axis::Vertical => (cell.rect.width()/n as f32, cell.rect.height(), 1.0, 0.0),
            _ => todo!(),
        };

        for j in 0..n + 1 {
            let a = Vec2::new(x + w * p * j as f32,  y + h * q * j as f32);
            let b = a + Vec2::new(w, h);
            let rect = Rect::new(a, b);
            let new_cell = RtCell {rect, neighbors: Vec::new()};
            if j == 0 {
                self.cells[i] = new_cell;
            } else {
                self.cells.push(new_cell)
            }
        }
        // TODO: NEIGHBORS!
    }

    /// Builds the graph.
    pub fn build<N, E, C, Ix>(self) -> VectorGraph<N, E, C, Ix>
    where
        Ix: IndexType,
    {
        let mut graph = VectorGraph::new();

        for i in 0..self.cells.len() {
            let cell = &self.cells[i];
            let rect = cell.rect;

            // Lets be naughty and pre-emptively assign cell index.
            let cell_index = CellIndex::new(graph.cells().len());
            let mut edges = Vec::new();

            for n in 0..cell.neighbors.len() {
                let other = self.cells[cell.neighbors[n]].rect;
                if let Some(line) = get_intersecting_line(rect, other) {
                    let node_a = graph.get_node(line.a);
                    let node_b = graph.get_node(line.b);
                    if let Some(edge) = graph.try_get_edge(node_a, node_b, true) {
                        graph.edge_mut(edge).unwrap().try_add_cell(cell_index);
                        edges.push(edge);
                    } else {
                        let edge =
                            graph.add_edge(Edge::new_single_cell(node_a, node_b, cell_index, None));
                        edges.push(edge);
                    }
                }
            }
            
            let actual_index = graph.add_cell(Cell::new(edges, None));
            assert_eq!(actual_index, cell_index, "Failed to build graph: assumed cell index was {} but graph used {}.", cell_index, actual_index);
        }
        return graph;
    }
}

fn get_y_line(x: f32, rect_a: Rect, rect_b: Rect) -> Line2 {
    let y_a = if rect_b.min.y > rect_a.min.y {
        rect_a.min.y
    } else {
        rect_b.min.y
    };
    let y_b = if rect_a.max.y < rect_b.max.y {
        rect_a.max.y
    } else {
        rect_b.max.y
    };
    return Line2::new(Vec2::new(x, y_a), Vec2::new(x, y_b));
}

fn get_x_line(y: f32, rect_a: Rect, rect_b: Rect) -> Line2 {
    let x_a = if rect_b.min.x > rect_a.min.x {
        rect_a.min.y
    } else {
        rect_b.min.y
    };
    let x_b = if rect_a.max.x < rect_b.max.x {
        rect_a.max.x
    } else {
        rect_b.max.x
    };
    return Line2::new(Vec2::new(x_a, y), Vec2::new(x_b, y));
}

fn assert_y_overlap(rect_a: Rect, rect_b: Rect) -> bool {
    (rect_a.min.y < rect_b.max.y && rect_a.min.y >= rect_b.min.y)
        || (rect_b.min.y < rect_a.max.y && rect_b.min.y >= rect_a.min.y)
}

fn assert_x_overlap(rect_a: Rect, rect_b: Rect) -> bool {
    (rect_a.min.x < rect_b.max.x && rect_a.min.x >= rect_b.min.x)
        || (rect_b.min.x < rect_a.max.x && rect_b.min.x >= rect_a.min.x)
}

fn get_intersecting_line(rect_a: Rect, rect_b: Rect) -> Option<Line2> {
    // left edge
    if rect_a.min.x == rect_b.max.x {
        if assert_y_overlap(rect_a, rect_b) {
            return Some(get_y_line(rect_a.min.x, rect_a, rect_b));
        }
        return None;
    }

    // right edge
    if rect_a.max.x == rect_b.min.x {
        if assert_y_overlap(rect_a, rect_b) {
            return Some(get_y_line(rect_a.max.x, rect_a, rect_b));
        }
        return None;
    }

    // top
    if rect_a.max.y == rect_b.min.y {
        if assert_x_overlap(rect_a, rect_b) {
            return Some(get_x_line(rect_a.max.y, rect_a, rect_b));
        }
        return None;
    }

    // bottom
    if rect_a.min.y == rect_b.max.y {
        if assert_x_overlap(rect_a, rect_b) {
            return Some(get_x_line(rect_a.min.y, rect_a, rect_b));
        }
        return None;
    }
    return None;
}


#[cfg(test)]
mod tests {
    use crate::{core::Axis::*, geom::{Rect, Vec2}, graphs::{RectTree, VectorGraph}};

    fn build_test_graph() -> VectorGraph<u8, u8, u8, u8> {
        let mut builder = RectTree::new(Rect::new(Vec2::ZERO, Vec2::splat(1080.0)));
        builder.split(0, 0.25, Horizontal);
        builder.split_n(1, 2, Vertical);
        builder.split(2, 0.75, Vertical);
        builder.split_n(6, 2, Horizontal);
        builder.split(8, 0.75, Vertical);
        builder.build()
    }

    #[test]
    fn treegraph_test() {
        let graph = build_test_graph();

        assert_eq!(graph.cells().len(), 8);
        assert_eq!(graph.edges().len(), 25);
        assert_eq!(graph.nodes().len(), 18);
    }

}

