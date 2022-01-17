use crate::{
    core::{Axis, DefaultIx, IndexType},
    geom::{Line2, Rect, Vec2, Intersect},
    graphs::Edge,
};

use super::{Cell, CellIndex, Node, VectorGraph};

#[derive(Debug)]
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
    pub fn split(&mut self, i: usize, position: f32, axis: Axis) -> &mut Self {
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
        self
    }

    /// Splits the given cell into n cells.
    pub fn split_n(&mut self, i: usize, n: usize, axis: Axis) -> &mut Self {
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
        self
    }

    /// Converts the RectTree into a [`VectorGraph`]- the generic graph type of this crate.
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

            for n in 0..self.cells[i].neighbours.len() {
                let j = self.cells[i].neighbours[n];
                let other = &self.cells[j];
                if let Some(intersection) = rect.get_touching_region(&other.rect) {
                    let node_a = Node::<D, Ix>::new(intersection.a, Vec::new(), None);
                    let node_b = Node::<D, Ix>::new(intersection.b, Vec::new(), None);
                    let node_a_index = graph.add_or_update_node(node_a);
                    let node_b_index = graph.add_or_update_node(node_b);
                    let edge = Edge::<D, Ix>::new(
                        node_a_index,
                        node_b_index,
                        cell.index.unwrap(),
                        other.index.unwrap(),
                        None,
                    );
                    graph.add_edge(edge);
                }
            }
        }

        // Complete the graph by adding non-linking edges.
        for i in 0..self.cells.len() {
            let cell_index = self.cells[i].index.unwrap();
            let mut raw_edges: Vec<Line2> = self.cells[i].rect.edges().into_iter().collect();
            let cell = graph.cells.get(&cell_index).unwrap();
            for j in cell.edges.iter() {
                let edge = graph.edge_line(*j).unwrap();
                raw_edges = Line2::subtract_collection(raw_edges.clone(), edge);
            }
            for edge in raw_edges {
                let node_a = graph.add_or_update_node(Node::<D, Ix>::new(edge.a, Vec::new(), None));
                let node_b = graph.add_or_update_node(Node::<D, Ix>::new(edge.b, Vec::new(), None));
                let edge = Edge::<D, Ix>::new_single_cell(node_a, node_b, cell_index, None);
                graph.add_edge(edge);
            }
        }

        return graph;
    }

    /// Returns the [Rect] at given index.
    pub fn get_rect(&self, i: usize) -> Rect {
        self.cells[i].rect
    }

    /// Returns the neighbors of the given [Rect] index.
    pub fn get_neighbours(&self, i: usize) -> Vec<Rect> {
        self.cells[i]
            .neighbours
            .iter()
            .map(|j| self.cells[*j].rect)
            .collect()
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

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn rect_tree_test() {
        use super::*;
        let mut rt =
            RectTree::<usize>::new(Rect::new(Vec2::new(0.0, 0.0), Vec2::new(100.0, 100.0)));
        rt.split(0, 0.5, Axis::Vertical);

        for c in rt.cells.iter() {
            println!("RtCell {:?}", c.rect);
        }

        let graph = rt.build::<f32>();
        for c in graph.cells.iter() {
            println!("Cell {:?}", c);
        }
        for e in graph.edges.iter() {
            println!("Edge {:?}", e);
        }

        for n in graph.nodes.iter() {
            println!("Node {:?}", n);
        }
    }
}
