use crate::geom::{Axis, Rect, Line, Transverse};
use num::{Num, Float};
use std::ops::{DivAssign, AddAssign};
use vek::Vec2;
use ordered_float::OrderedFloat;


struct TreeRect<T> where T: Float + Copy + DivAssign + AddAssign {
    rect: Rect<T, T>,
    parent: Option<usize>,
    children: Vec<usize>,
    edges: Vec<(usize, Transverse)>,
}

#[derive(Debug, Copy, Clone)]
struct TreeEdge<T> where T: Float + Copy + DivAssign + AddAssign {
    a: usize,
    b: usize,
    line: Line<T>,
    axis: Axis,
    active: bool,
}

pub struct Treemap<T> where T: Float + Copy + DivAssign + AddAssign {
    rects: Vec<TreeRect<T>>,
    edges: Vec<TreeEdge<T>>,
}


impl<T> TreeRect<T> where T: Float + Copy + DivAssign + AddAssign {
    fn new(rect: Rect<T, T>, parent: Option<usize>) -> Self {
        Self {
            rect,
            parent,
            children: Vec::new(),
            edges: Vec::new(),
        }
    }
}

impl<T> Treemap<T> where T: Float + Copy + DivAssign + AddAssign {
    pub fn new(rect: Rect<T, T>) -> Self {
        Self {
            rects: vec![TreeRect::new(rect, None)],
            edges: Vec::new(),
        }
    }

    pub fn split(&mut self, index: usize, axis: Axis) -> (usize, usize) {

        let index_a = self.rects.len();
        let index_b = index_a + 1;

        let (a, b, line, transverse) = match axis {
            Axis::Horizontal => {
                let x = self.rects[index].rect.x;
                let w = self.rects[index].rect.w;
                let y = self.rects[index].rect.y;
                let h = self.rects[index].rect.h / (T::one() + T::one());
                let rect_top = Rect::new(x, h, w, h);
                let rect_bottom = Rect::new(x, y, w, h);

                let line = Line {
                    start: Vec2::new(x, y + h),
                    end: Vec2::new(x + w, y + h),
                };
                
                (rect_bottom, rect_top, line, Transverse::Left)
            },
            Axis::Vertical => {
                let x = self.rects[index].rect.x;
                let w = self.rects[index].rect.w  / (T::one() + T::one());
                let y = self.rects[index].rect.y;
                let h = self.rects[index].rect.h;
                let rect_left = Rect::new(x, y, w, h);
                let rect_right = Rect::new(w, y, w, h);

                let line =  Line {
                    start: Vec2::new(x, y + h),
                    end: Vec2::new(x + w, y + h),
                };

                (rect_left, rect_right, line, Transverse::Down)
            },
            _ => {
                panic!();
            }
        };

        let edge_index = self.edges.len();
        self.edges.push(TreeEdge {
            a: index_a,
            b: index_b,
            line,
            axis,
            active: true,
        });

        let mut node_a = TreeRect::new(a, Some(index));
        let mut node_b = TreeRect::new(b, Some(index));

        node_a.edges.push((edge_index, transverse));
        node_b.edges.push((edge_index, transverse.opposite()));

        self.rects[index].children.push(index_a);
        self.rects[index].children.push(index_b);
        self.rects.push(node_a);
        self.rects.push(node_b);

        self.inherit_edges(index_a, transverse);
        self.inherit_edges(index_b, transverse.opposite());

        (index_a, index_b)
    }

    fn inherit_edges(&mut self, index: usize, applied_edge: Transverse) {
        if let Some(parent_index) = self.rects[index].parent {
            for (edge_index, transverse) in self.rects[parent_index].edges.iter() {
                if *transverse != applied_edge {
                    // Inherit edge:
                    // - deactivate old edge
                    // - create new one
                    let mut new_edge = self.edges[*edge_index].clone();
                    self.edges[*edge_index].active = false;

                    if new_edge.a == parent_index {
                        new_edge.a = index;
                    } else if new_edge.b == parent_index {
                        new_edge.b = index;
                    }

                    let a_x1 = OrderedFloat(self.rects[new_edge.a].rect.x);
                    let b_x1 = OrderedFloat(self.rects[new_edge.b].rect.x);
                    let a_x2 = OrderedFloat(self.rects[new_edge.a].rect.x + self.rects[new_edge.a].rect.w);
                    let b_x2 = OrderedFloat(self.rects[new_edge.b].rect.x + self.rects[new_edge.b].rect.w);

                    let a_y1 = OrderedFloat(self.rects[new_edge.a].rect.y);
                    let b_y1 = OrderedFloat(self.rects[new_edge.b].rect.y);
                    let a_y2 = OrderedFloat(self.rects[new_edge.a].rect.y + self.rects[new_edge.a].rect.h);
                    let b_y2 = OrderedFloat(self.rects[new_edge.b].rect.y + self.rects[new_edge.b].rect.h);

                    let (x1, x2, y1, y2) = match transverse {
                        Transverse::Up => {
                            let x1 = a_x1.max(b_x1).into_inner();
                            let x2 = a_x2.min(b_x2).into_inner();
                            let y = self.rects[new_edge.b].rect.y + self.rects[new_edge.b].rect.h;
                            (x1, x2, y, y)
                        },
                        Transverse::Down => {
                            let x1 = a_x1.max(b_x1).into_inner();
                            let x2 = a_x2.min(b_x2).into_inner();
                            let y = self.rects[new_edge.b].rect.y;
                            (x1, x2, y, y)
                        },
                        Transverse::Left => {
                            let x = self.rects[new_edge.a].rect.x;
                            let y1 = a_y1.max(b_y1).into_inner();
                            let y2 = a_y1.min(b_y2).into_inner();
                            (x, x, y1, y2)
                        },
                        Transverse::Right => {
                            let x = self.rects[new_edge.a].rect.x + self.rects[new_edge.a].rect.w;
                            let y1 = a_y1.max(b_y1).into_inner();
                            let y2 = a_y1.min(b_y2).into_inner();
                            (x, x, y1, y2)
                        },
                        _ => {
                            panic!("Transverse for anything other than udlr is wrong and you shouldnt do it");
                        }
                    };

                    new_edge.line = Line {
                        start: Vec2::new(x1, y1),
                        end: Vec2::new(x2, y2),
                    };
                    self.edges.push(new_edge);
                }
            }
        }
    }
}

#[test]
fn treemap_test() {
    let _tree_map = Treemap::<f32>::new(Rect::new(0., 0., 64., 64.));
}