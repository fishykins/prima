use crate::{
    core::{Axis, DefaultIx, IndexType},
    geom::{Rect, Vec2},
    graphs::Edge,
};

use super::{Cell, CellIndex, Node, VectorGraph};

struct RtCell<Ix = DefaultIx>
where
    Ix: IndexType,
{
    rect: Rect,
    neighbours: Vec<usize>,
    index: Option<CellIndex<Ix>>,
}

/// A Rect tree graph.
pub struct RectTree<Ix = DefaultIx>
where
    Ix: IndexType,
{
    cells: Vec<RtCell<Ix>>,
}

impl<Ix> RectTree<Ix>
where
    Ix: IndexType,
{
    /// Builds a new RectTree graph
    pub fn new(root: Rect) -> Self {
        let cell = RtCell::<Ix> {
            rect: root,
            neighbours: Vec::new(),
            index: None,
        };
        Self { cells: vec![cell] }
    }

    /// Splits the given cell in two.
    pub fn split(&mut self, i: usize, position: f32, axis: Axis) {
        let j = self.cells.len();
        let old_cell = &self.cells[i];
        let (a, b) = old_cell.rect.split(position, axis);
        let cell_a = RtCell {
            index: None,
            rect: a,
            neighbours: old_cell
                .neighbours
                .clone()
                .into_iter()
                .chain(vec![j].into_iter())
                .collect(),
        };
        let cell_b = RtCell {
            index: None,
            rect: b,
            neighbours: old_cell
                .neighbours
                .clone()
                .into_iter()
                .chain(vec![i].into_iter())
                .collect(),
        };

        self.cells[i] = cell_a;
        self.cells.push(cell_b);
    }

    /// Splits the given cell into n cells.
    pub fn split_n(&mut self, i: usize, n: usize, axis: Axis) {
        let cell = &self.cells[i];
        let x = cell.rect.min.x;
        let y = cell.rect.min.y;
        let (w, h, p, q) = match axis {
            Axis::Horizontal => (cell.rect.width(), cell.rect.height() / n as f32, 0.0, 1.0),
            Axis::Vertical => (cell.rect.width() / n as f32, cell.rect.height(), 1.0, 0.0),
            _ => todo!(),
        };

        let neighbour_cache = cell.neighbours.clone();

        for j in 0..n + 1 {
            let a = Vec2::new(x + w * p * j as f32, y + h * q * j as f32);
            let b = a + Vec2::new(w, h);
            let rect = Rect::new(a, b);
            let mut neighbours = neighbour_cache.clone();
            if j > 1 {
                neighbours.push(j - 1);
            } else if j == 1 {
                neighbours.push(i);
            }
            if j < n + 1 {
                neighbours.push(j + 1);
            }
            let new_cell = RtCell {
                rect,
                neighbours,
                index: None,
            };
            if j == 0 {
                self.cells[i] = new_cell;
                self.filter_neighbors(i);
            } else {
                self.cells.push(new_cell);
            }
        }
    }

    /// Converts the RectTree into a [`VectorGrpah`]- the generic graph type of this crate.
    pub fn build<D>(mut self) -> VectorGraph<D, Ix>
    where
        Ix: IndexType,
    {
        let mut graph = VectorGraph::<D, Ix>::new();

        for c in self.cells.iter_mut() {
            // Add cells to graph without any data attached. We can add this in a later loop.
            let cell = Cell::<D, Ix>::new(Vec::new(), None);
            let index = graph.add_cell(cell);
            c.index = Some(index);
        }

        for i in 0..self.cells.len() {
            self.filter_neighbors(i);

            let cell = &self.cells[i];
            let rect = cell.rect;

            for n in 0..cell.neighbours.len() {
                let other = &self.cells[cell.neighbours[n]];
                // Build edges between cells
                if rect.max.x == other.rect.min.x {
                    let a = Vec2::new(rect.max.x, f32::max(rect.min.y, other.rect.min.y));
                    let b = Vec2::new(rect.max.x, f32::min(rect.max.y, other.rect.max.y));
                    let node_a = Node::<D, Ix>::new(a, Vec::new(), None);
                    let node_b = Node::<D, Ix>::new(b, Vec::new(), None);
                    let node_a_index = graph.add_node(node_a);
                    let node_b_index = graph.add_node(node_b);
                    let edge = Edge::<D, Ix>::new(
                        node_a_index,
                        node_b_index,
                        cell.index.unwrap(),
                        other.index.unwrap(),
                        None,
                    );
                    let edge_index = graph.add_edge(edge);
                    graph
                        .cells
                        .get_mut(&cell.index.unwrap())
                        .unwrap()
                        .edges
                        .push(edge_index);
                } else if rect.max.y == other.rect.min.y {
                    let a = Vec2::new(f32::max(rect.min.x, other.rect.min.x), rect.max.y);
                    let b = Vec2::new(f32::min(rect.max.x, other.rect.max.x), rect.max.y);
                    Self::add_cell_to_graph(cell, other, a, b, &mut graph);
                }
            }
        }
        return graph;
    }

    /// Asseses a cells neighbors and removes any that are infact, not.
    fn filter_neighbors(&mut self, i: usize) {
        let cell = &self.cells[i];
        let mut valid = Vec::new();
        for j in cell.neighbours.iter() {
            if *j == i {
                continue;
            }
            let other = &self.cells[*j];
            if cell.rect.intersects(&other.rect) {
                valid.push(*j);
            }
        }
        self.cells[i].neighbours = valid;
    }

    /// Take a cell and constructs an edge between it and the other cell, before adding it to the graph.
    fn add_cell_to_graph<D>(
        cell: &RtCell<Ix>,
        other: &RtCell<Ix>,
        a: Vec2,
        b: Vec2,
        graph: &mut VectorGraph<D, Ix>,
    ) {
        let node_a = Node::<D, Ix>::new(a, Vec::new(), None);
        let node_b = Node::<D, Ix>::new(b, Vec::new(), None);
        let node_a_index = graph.add_node(node_a);
        let node_b_index = graph.add_node(node_b);
        let edge = Edge::<D, Ix>::new(
            node_a_index,
            node_b_index,
            cell.index.unwrap(),
            other.index.unwrap(),
            None,
        );
        let edge_index = graph.add_edge(edge);
        graph
            .cells
            .get_mut(&cell.index.unwrap())
            .unwrap()
            .edges
            .push(edge_index);
        graph
            .cells
            .get_mut(&other.index.unwrap())
            .unwrap()
            .edges
            .push(edge_index);
    }
}
