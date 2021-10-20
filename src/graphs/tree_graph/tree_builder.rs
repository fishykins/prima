// A constructor class for TreeMap
use super::{EdgeRef, TreeEdge, TreeGraph, TreeRect};
use crate::core::maths::clamp01;
use crate::core::{IndexType, OrdNum};
use crate::geom::{Axis, Line, Rect, Transverse};
use crate::graphs::{Cell, Edge, Node};
use num::Float;
use ordered_float::OrderedFloat;
use std::collections::HashMap;
use vek::Vec2;

pub struct TreeBuilder<T>
where
    T: OrdNum + Float,
{
    rects: Vec<TreeRect<T>>,
    edges: Vec<TreeEdge<T>>,
    cycle: usize,
}

impl<T> TreeBuilder<T>
where
    T: OrdNum + Float,
{
    pub fn new(rect: Rect<T, T>) -> Self {
        TreeBuilder {
            rects: vec![TreeRect::new(rect, None)],
            edges: Vec::new(),
            cycle: 0,
        }
    }

    pub fn build_graph<C, E, N, Ix>(self) -> TreeGraph<T, C, E, N, Ix>
    where
        T: Clone,
        C: Clone,
        E: Clone,
        N: Clone,
        Ix: Clone + IndexType,
    {
        // Get a list of all active rects and edges
        let active_rects: Vec<(usize, &TreeRect<T>)> = self
            .rects
            .iter()
            .enumerate()
            .filter(|(_, x)| x.active())
            .collect();

        let active_edges: Vec<(usize, &TreeEdge<T>)> = self
            .edges
            .iter()
            .enumerate()
            .filter(|(_, x)| x.active())
            .collect();

        // (old index, new index)
        let mut rect_hashmap = HashMap::<usize, usize>::new();
        let mut edge_hashmap = HashMap::<usize, usize>::new();

        for (i, r) in active_rects.iter().enumerate() {
            rect_hashmap.insert(r.0, i);
        }
        for (i, e) in active_edges.iter().enumerate() {
            edge_hashmap.insert(e.0, i);
        }

        let mut rects = Vec::<Rect<T, T>>::new();
        let mut cells = Vec::<Cell<C, Ix>>::new();
        let mut edges = Vec::<Edge<E, Ix>>::new();
        let mut nodes = Vec::<Node<T, N, Ix>>::new();
        // Remap all values in every active rect to a cell
        for (_, r) in active_rects.iter() {
            let edges = r
                .edges()
                .iter()
                .filter(|e| edge_hashmap.contains_key(&e.0))
                .map(|e| IndexType::new(*edge_hashmap.get(&e.0).unwrap()))
                .collect();
            rects.push(r.rect.clone());
            cells.push(Cell::<C, Ix>::new(edges, None));
        }

        // All nodes, cells and edges are linked up propperly via this for loop
        for (i, e) in active_edges.iter().enumerate() {
            let old_edge = e.1;

            // Get an index for both nodes and both cells
            let node_ai: usize = Self::get_node(&mut nodes, old_edge.line.start);
            let node_bi: usize = Self::get_node(&mut nodes, old_edge.line.end);
            let cell_ai: usize = *rect_hashmap.get(&old_edge.a).unwrap();
            let cell_bi: usize = *rect_hashmap.get(&old_edge.b).unwrap();

            // Now we have both our nodes, update their values.
            nodes[node_ai].linked_edges.push(IndexType::new(i));
            nodes[node_bi].linked_edges.push(IndexType::new(i));

            // Finally, create a new edge
            edges.push(Edge {
                node_a: IndexType::new(node_ai),
                node_b: IndexType::new(node_bi),
                cell_a: IndexType::new(cell_ai),
                cell_b: IndexType::new(cell_bi),
                data: None,
            });
        }

        // Go round the actual edges of the map and add them in.
        for i in 0..cells.len() {
            let cell = &mut cells[i];
            let rect = &rects[i];
            let mut up = false;
            let mut right = false;
            let mut down = false;
            let mut left = false;

            let x1 = rect.x;
            let x2 = rect.x + rect.w;
            let y1 = rect.y;
            let y2 = rect.y + rect.h;

            for e in cell.edges().iter() {
                let edge = &edges[e.index()];
                let node_a = &nodes[edge.node_a.index()];
                let node_b = &nodes[edge.node_b.index()];
                if node_a.pos().x == x1 && node_b.pos().x == x1 {
                    left = true;
                    continue;
                }
                if node_a.pos().x == x2 && node_b.pos().x == x2 {
                    right = true;
                    continue;
                }
                if node_a.pos().y == y1 && node_b.pos().y == y1 {
                    down = true;
                    continue;
                }
                if node_a.pos().y == y2 && node_b.pos().y == y2 {
                    up = true;
                }
            }

            let cell_i = IndexType::new(i);

            if !up {
                // No edge on top
                let node_ai = IndexType::new(Self::get_node(&mut nodes, Vec2::new(x1, y2)));
                let node_bi = IndexType::new(Self::get_node(&mut nodes, Vec2::new(x2, y2)));
                let edge_i = edges.len();
                edges.push(Edge::<E, Ix>::new(node_ai, node_bi, cell_i, cell_i, None));
                cells[i].edges.push(IndexType::new(edge_i));
            }

            if !right {
                // No edge on the right
                let node_ai = IndexType::new(Self::get_node(&mut nodes, Vec2::new(x2, y1)));
                let node_bi = IndexType::new(Self::get_node(&mut nodes, Vec2::new(x2, y2)));
                let edge_i = edges.len();
                edges.push(Edge::<E, Ix>::new(node_ai, node_bi, cell_i, cell_i, None));
                cells[i].edges.push(IndexType::new(edge_i));
            }

            if !down {
                // No edge down
                let node_ai = IndexType::new(Self::get_node(&mut nodes, Vec2::new(x1, y1)));
                let node_bi = IndexType::new(Self::get_node(&mut nodes, Vec2::new(x2, y1)));
                let edge_i = edges.len();
                edges.push(Edge::<E, Ix>::new(node_ai, node_bi, cell_i, cell_i, None));
                cells[i].edges.push(IndexType::new(edge_i));
            }
            if !left {
                // No edge on the left
                let node_ai = IndexType::new(Self::get_node(&mut nodes, Vec2::new(x1, y1)));
                let node_bi = IndexType::new(Self::get_node(&mut nodes, Vec2::new(x1, y2)));
                let edge_i = edges.len();
                edges.push(Edge::<E, Ix>::new(node_ai, node_bi, cell_i, cell_i, None));
                cells[i].edges.push(IndexType::new(edge_i));
            }
        }

        // Create final node, edge and cell structs

        TreeGraph {
            cells,
            nodes,
            edges,
            rects,
        }
    }

    pub fn rect(&mut self, index: usize) -> Option<&TreeRect<T>> {
        if index < self.rects.len() {
            return Some(&self.rects[index]);
        }
        None
    }

    pub fn edge(&self, index: usize) -> Option<&TreeEdge<T>> {
        if index < self.edges.len() {
            return Some(&self.edges[index]);
        }
        None
    }

    pub fn edge_mut(&mut self, index: usize) -> Option<&TreeEdge<T>> {
        if index < self.edges.len() {
            return Some(&self.edges[index]);
        }
        None
    }

    /// Getter for all active rectangles
    pub fn rects(&self) -> Vec<usize> {
        self.rects
            .iter()
            .enumerate()
            .filter(|(_, x)| x.active())
            .map(|(i, _)| i)
            .collect()
    }

    /// Getter for all active edges
    pub fn edges(&self) -> Vec<usize> {
        self.edges
            .iter()
            .enumerate()
            .filter(|(_, x)| x.active())
            .map(|(i, _)| i)
            .collect()
    }

    pub fn intersect_point(&mut self, index: usize, axis: Axis, offset: T) -> bool {
        if index >= self.rects.len() {
            panic!(
                "Index {} falls outside the range of {}: intersection will fail",
                index,
                self.rects.len()
            );
        }

        if !self.rects[index].active() {
            return false;
        }

        let clamped_offset = clamp01(offset);
        let x_a = self.rects[index].rect.x;
        let y_a = self.rects[index].rect.y;
        let w_a: T;
        let h_a: T;

        let x_b: T;
        let y_b: T;
        let h_b: T;
        let w_b: T;
        let transverse: Transverse;
        let split_line: Line<T>;

        match axis {
            Axis::Horizontal => {
                w_a = self.rects[index].rect.w;
                h_a = self.rects[index].rect.h * clamped_offset;
                x_b = x_a;
                y_b = y_a + h_a;
                w_b = w_a;
                h_b = self.rects[index].rect.h - h_a;
                transverse = Transverse::Up;
                split_line = Line {
                    start: Vec2::new(x_a, y_a + h_a),
                    end: Vec2::new(x_a + w_a, y_a + h_a),
                };
            }
            Axis::Vertical => {
                w_a = self.rects[index].rect.w * clamped_offset;
                h_a = self.rects[index].rect.h;
                x_b = x_a + w_a;
                y_b = y_a;
                w_b = self.rects[index].rect.w - w_a;
                h_b = h_a;
                transverse = Transverse::Right;
                split_line = Line {
                    start: Vec2::new(x_a + w_a, y_a),
                    end: Vec2::new(x_a + w_a, y_a + h_a),
                };
            }
            _ => {
                panic!("No axis")
            }
        };

        let rect_a = Rect::<T, T>::new(x_a, y_a, w_a, h_a);
        let rect_b = Rect::<T, T>::new(x_b, y_b, w_b, h_b);
        let index_a = self.rects.len();
        let index_b = index_a + 1;

        let edge_index = self.edges.len();
        self.edges.push(TreeEdge {
            a: index_a,
            b: index_b,
            line: split_line.clone(),
            axis,
            birth_cycle: None,
        });

        self.rects.push(TreeRect::new(rect_a, Some(index)));
        self.rects.push(TreeRect::new(rect_b, Some(index)));
        self.rects[index].children.push(index_a);
        self.rects[index].children.push(index_b);
        self.rects[index_a]
            .edges
            .push(EdgeRef(edge_index, transverse));
        self.rects[index_b]
            .edges
            .push(EdgeRef(edge_index, transverse.opposite()));
        self.inherit_edges(index_a, transverse);
        self.inherit_edges(index_b, transverse.opposite());

        self.cycle += 1;
        true
    }

    /// Splits a node into n slices, where n is the number of cuts
    pub fn split(&mut self, index: usize, axis: Axis, cuts: usize) -> Vec<usize> {
        if index >= self.rects.len() {
            return Vec::new();
        }

        if !self.rects[index].active() {
            return Vec::new();
        }

        let new_rects = Vec::new();

        let n = T::from_usize(cuts + 1).unwrap();
        let x = self.rects[index].rect.x;
        let y = self.rects[index].rect.y;

        let (w, h, p, q, transverse) = match axis {
            Axis::Horizontal => (
                self.rects[index].rect.w,
                self.rects[index].rect.h / n,
                T::zero(),
                T::one(),
                Transverse::Up,
            ),
            Axis::Vertical => (
                self.rects[index].rect.w / n,
                self.rects[index].rect.h,
                T::one(),
                T::zero(),
                Transverse::Right,
            ),
            _ => {
                panic!("No axis")
            }
        };

        for i in 0..cuts + 1 {
            let j: T = T::from_usize(i).unwrap();
            let rect = Rect::<T, T>::new(x + w * p * j, y + h * q * j, w, h);
            let index_b = self.rects.len();
            self.rects.push(TreeRect::new(rect, Some(index)));
            self.rects[index].children.push(index_b);
            if i > 0 {
                let index_a = index_b - 1;
                let line = Line {
                    start: Vec2::new(x + w * j * p, y + h * j * q),
                    end: Vec2::new(x + w * j * p + w * q, y + h * j * q + h * p),
                };
                let edge_index = self.edges.len();
                self.edges.push(TreeEdge {
                    a: index_a,
                    b: index_b,
                    line: line.clone(),
                    axis,
                    birth_cycle: None,
                });

                self.rects[index_a]
                    .edges
                    .push(EdgeRef(edge_index, transverse));
                self.rects[index_b]
                    .edges
                    .push(EdgeRef(edge_index, transverse.opposite()));
                self.inherit_edges(index_a, transverse);

                if i == cuts {
                    self.inherit_edges(index_b, transverse.opposite());
                }
            }
        }

        self.cycle += 1;
        new_rects
    }

    /// Allows a newly formed rect to inherit a parents edges
    fn inherit_edges(&mut self, rect_index: usize, applied_edge: Transverse) {
        if let Some(parent_index) = self.rects[rect_index].parent {
            let mut edges_to_add = Vec::<(usize, EdgeRef)>::new();

            for EdgeRef(edge_index, transverse) in self.rects[parent_index].edges.iter() {
                if *transverse != applied_edge && self.edges[*edge_index].can_split(self.cycle) {
                    let new_edge_index = self.edges.len();
                    let mut new_edge = self.edges[*edge_index].clone();

                    let other;

                    if new_edge.a == parent_index {
                        new_edge.a = rect_index;
                        other = new_edge.b;
                    } else if new_edge.b == parent_index {
                        new_edge.b = rect_index;
                        other = new_edge.a;
                    } else {
                        panic!("Edge indexed does not belong to parent- this is bad");
                    }

                    let a_x1 = OrderedFloat(self.rects[new_edge.a].rect.x);
                    let b_x1 = OrderedFloat(self.rects[new_edge.b].rect.x);
                    let a_x2 =
                        OrderedFloat(self.rects[new_edge.a].rect.x + self.rects[new_edge.a].rect.w);
                    let b_x2 =
                        OrderedFloat(self.rects[new_edge.b].rect.x + self.rects[new_edge.b].rect.w);

                    let a_y1 = OrderedFloat(self.rects[new_edge.a].rect.y);
                    let b_y1 = OrderedFloat(self.rects[new_edge.b].rect.y);
                    let a_y2 =
                        OrderedFloat(self.rects[new_edge.a].rect.y + self.rects[new_edge.a].rect.h);
                    let b_y2 =
                        OrderedFloat(self.rects[new_edge.b].rect.y + self.rects[new_edge.b].rect.h);

                    let valid: bool;

                    let (x1, x2, y1, y2) = match transverse {
                        Transverse::Up => {
                            let x1 = Ord::max(a_x1, b_x1).into_inner();
                            let x2 = Ord::min(a_x2, b_x2).into_inner();
                            let y = self.rects[new_edge.b].rect.y; // + self.rects[new_edge.b].rect.h;
                            valid = self.validate_horizontal(new_edge.a, new_edge.b);
                            (x1, x2, y, y)
                        }
                        Transverse::Down => {
                            let x1 = Ord::max(a_x1, b_x1).into_inner();
                            let x2 = Ord::min(a_x2, b_x2).into_inner();
                            let y = self.rects[new_edge.b].rect.y;
                            valid = self.validate_horizontal(new_edge.a, new_edge.b);
                            (x1, x2, y, y)
                        }
                        Transverse::Left => {
                            let x = self.rects[new_edge.b].rect.x;
                            let y1 = Ord::max(a_y1, b_y1).into_inner();
                            let y2 = Ord::min(a_y2, b_y2).into_inner();
                            valid = self.validate_vertical(new_edge.a, new_edge.b);
                            (x, x, y1, y2)
                        }
                        Transverse::Right => {
                            let x = self.rects[new_edge.b].rect.x;
                            let y1 = Ord::max(a_y1, b_y1).into_inner();
                            let y2 = Ord::min(a_y2, b_y2).into_inner();
                            valid = self.validate_vertical(new_edge.a, new_edge.b);
                            (x, x, y1, y2)
                        }
                        _ => {
                            panic!("Transverse for anything other than udlr is wrong and you shouldnt do it");
                        }
                    };

                    if valid {
                        new_edge.line = Line {
                            start: Vec2::new(x1, y1),
                            end: Vec2::new(x2, y2),
                        };

                        self.edges.push(new_edge);
                        edges_to_add
                            .push((rect_index, EdgeRef(new_edge_index, transverse.clone())));
                        edges_to_add.push((other, EdgeRef(new_edge_index, transverse.opposite())));
                        self.edges[*edge_index].birth_cycle = Some(self.cycle);
                        self.edges[new_edge_index].birth_cycle = None;
                    }
                }
            }
            for (i, eref) in edges_to_add {
                self.rects[i].edges.push(eref);
            }
        }
    }

    fn validate_horizontal(&self, a: usize, b: usize) -> bool {
        let rect_a = &self.rects[a].rect;
        let rect_b = &self.rects[b].rect;
        let rect_a_x2 = rect_a.x + rect_a.w;
        let rect_b_x2 = rect_b.x + rect_b.w;
        let rect_a_y2 = rect_a.y + rect_a.h;
        let rect_b_y2 = rect_b.y + rect_b.h;

        let a_in_b = rect_a.x >= rect_b.x && rect_a_x2 <= rect_b_x2;
        let b_in_a = rect_b.x >= rect_a.x && rect_b_x2 <= rect_a_x2;
        let a_overlaps_left = rect_a_x2 > rect_b.x && rect_a_x2 < rect_b_x2;
        let a_overlaps_right = rect_a.x > rect_b.x && rect_a.x < rect_b_x2;

        (a_in_b || b_in_a || a_overlaps_left || a_overlaps_right)
            && (rect_a.y == rect_b_y2 || rect_a_y2 == rect_b.y)
    }

    fn validate_vertical(&self, a: usize, b: usize) -> bool {
        let rect_a = &self.rects[a].rect;
        let rect_b = &self.rects[b].rect;
        let rect_a_x2 = rect_a.x + rect_a.w;
        let rect_b_x2 = rect_b.x + rect_b.w;
        let rect_a_y2 = rect_a.y + rect_a.h;
        let rect_b_y2 = rect_b.y + rect_b.h;
        let a_in_b = rect_a.y >= rect_b.y && rect_a_y2 <= rect_b_y2;
        let b_in_a = rect_b.y >= rect_a.y && rect_b_y2 <= rect_a_y2;
        let a_overlaps_down = rect_a_y2 > rect_b.y && rect_a_y2 < rect_b_y2;
        let a_overlaps_up = rect_a.y > rect_b.y && rect_a.y < rect_b_y2;

        (a_in_b || b_in_a || a_overlaps_down || a_overlaps_up)
            && (rect_a.x == rect_b_x2 || rect_a_x2 == rect_b.x)
    }

    fn get_node<N, Ix>(nodes: &mut Vec<Node<T, N, Ix>>, pos: Vec2<T>) -> usize
    where
        Ix: IndexType,
    {
        let index: usize;
        if let Some(node_index_existing) = nodes
            .iter()
            .position(|n| n.pos().x == pos.x && n.pos().y == pos.y)
        {
            index = node_index_existing;
        } else {
            index = nodes.len();
            nodes.push(Node::new(pos.x, pos.y, Vec::new(), None));
        };
        return index;
    }
}
