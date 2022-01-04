use crate::{
    core::{Axis, IndexType},
    geom::{Rect, Vec2},
};

use super::{VectorGraph, CellIndex, Cell};

struct RtCell {
    rect: Rect,
    neighbours: Vec<usize>,
}

/// A Rect tree graph.
pub struct RectTree {
    cells: Vec<RtCell>,
}

impl RectTree {
    /// Builds a new RectTree graph
    pub fn new(root: Rect) -> Self {
        Self {
            cells: vec![RtCell {
                rect: root,
                neighbours: Vec::new(),
            }],
        }
    }

    /// Splits the given cell in two.
    pub fn split(&mut self, i: usize, position: f32, axis: Axis) {
        let j = self.cells.len();
        let old_cell = &self.cells[i];
        let (a, b) = old_cell.rect.split(position, axis);
        let cell_a = RtCell {
            rect: a,
            neighbours: old_cell
                .neighbours
                .clone()
                .into_iter()
                .chain(vec![j].into_iter())
                .collect(),
        };
        let cell_b = RtCell {
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
            let new_cell = RtCell { rect, neighbours };
            if j == 0 {
                self.cells[i] = new_cell;
                self.filter_neighbors(i);
            } else {
                self.cells.push(new_cell);
            }
        }
    }

    /// Converts the RectTree into a [`VectorGrpah`]- the generic graph type of this crate.
    pub fn build<N, E, C, Ix>(mut self) -> VectorGraph<N, E, C, Ix>
    where
        Ix: IndexType,
    {
        let mut graph = VectorGraph::new();

        for i in 0..self.cells.len() {
            self.filter_neighbors(i);

            let cell = &self.cells[i];
            let rect = cell.rect;

            // Lets be naughty and pre-emptively assign cell index.
            let cell_index = CellIndex::new(graph.cells().len());
            let mut edges = Vec::new();

            for n in 0..cell.neighbours.len() {
                let other = self.cells[cell.neighbours[n]].rect;
                // Build edges between cells
                
                
            }
            let actual_index = graph.add_cell(Cell::new(edges, None));
            assert_eq!(actual_index, cell_index, "Failed to build graph: assumed cell index was {} but graph used {}.", cell_index, actual_index);
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
}
