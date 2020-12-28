use crate::geom::{Axis, Rect, Line, Transverse};
use crate::render::{Draw, draw_text, ImageBuffer, RgbRaw, load_font};
use crate::core::{OrdNum, maths::clamp01};
use num::{Float};
use std::fmt::Debug;
use vek::{Vec2, Rgb};
use ordered_float::OrderedFloat;

#[derive(Debug, Copy, Clone)]
pub struct EdgeRef(usize, Transverse);

pub struct TreeRect<T> where T: OrdNum {
    rect: Rect<T, T>,
    parent: Option<usize>,
    children: Vec<usize>,
    edges: Vec<EdgeRef>,
}

#[derive(Debug, Copy, Clone)]
pub struct TreeEdge<T> where T: OrdNum + Float {
    a: usize,
    b: usize,
    line: Line<T>,
    axis: Axis,
    child_count: u8,
}

pub struct Treemap<T> where T: OrdNum + Float {
    rects: Vec<TreeRect<T>>,
    edges: Vec<TreeEdge<T>>,
}

impl<T> TreeEdge<T> where T: OrdNum + Float {
    fn active(&self) -> bool {
        self.child_count < 2
    }
}

impl<T> TreeRect<T> where T: OrdNum + Float {
    fn new(rect: Rect<T, T>, parent: Option<usize>) -> Self {
        Self {
            rect,
            parent,
            children: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn active(&self) -> bool {
        self.children.len() == 0
    }
}

impl<T> Treemap<T> where T: OrdNum + Float {
    pub fn new(rect: Rect<T, T>) -> Self {
        Self {
            rects: vec![TreeRect::new(rect, None)],
            edges: Vec::new(),
        }
    }

    pub fn rect(&mut self, index: usize) -> Option<&TreeRect<T>> {
        if index < self.rects.len() {
            return Some(&self.rects[index]);
        }
        None
    }

    pub fn edge(&mut self, index: usize) -> Option<&TreeEdge<T>> {
        if index < self.edges.len() {
            return Some(&self.edges[index]);
        }
        None
    }

    pub fn intersect_point(&mut self, index: usize, axis: Axis, offset: T) {
        if index > self.rects.len() {
            return;
        }

        if !self.rects[index].active() {
            return;
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
                    start:  Vec2::new(x_a, y_a + h_a),
                    end:    Vec2::new(x_a + w_a, y_a + h_a),
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
                    start:  Vec2::new(x_a + w_a, y_a),
                    end:    Vec2::new(x_a + w_a, y_a + h_a),
                };
            }
            _ => {panic!("No axis")},
        };

        let rect_a = Rect::<T,T>::new(x_a, y_a, w_a, h_a);
        let rect_b = Rect::<T,T>::new(x_b, y_b, w_b, h_b);
        let index_a = self.rects.len();
        let index_b = index_a + 1;

        let edge_index = self.edges.len();
        self.edges.push(TreeEdge {
            a: index_a,
            b: index_b,
            line: split_line.clone(),
            axis,
            child_count: 0,
        });

        self.rects.push(TreeRect::new(rect_a, Some(index)));
        self.rects.push(TreeRect::new(rect_b, Some(index)));
        self.rects[index].children.push(index_a);
        self.rects[index].children.push(index_b);
        self.rects[index_a].edges.push(EdgeRef(edge_index, transverse));
        self.rects[index_b].edges.push(EdgeRef(edge_index, transverse.opposite()));
        self.inherit_edges(index_a, transverse);
        self.inherit_edges(index_b, transverse.opposite());
    }

    pub fn split(&mut self, index: usize, axis: Axis, cuts: usize) -> Vec<usize> {

        if index > self.rects.len() {
            return Vec::new();
        }

        if !self.rects[index].active() {
            return Vec::new();
        }

        let new_rects= Vec::new();

        let n = T::from_usize(cuts + 1).unwrap();
        let x = self.rects[index].rect.x;
        let y = self.rects[index].rect.y;

        let (w, h, p, q, transverse) = match axis {
            Axis::Horizontal => (self.rects[index].rect.w, self.rects[index].rect.h / n, T::zero(), T::one(), Transverse::Right),
            Axis::Vertical => (self.rects[index].rect.w  / n, self.rects[index].rect.h, T::one(), T::zero(), Transverse::Up),
            _ => {panic!("No axis")},
        };

        for i in 0..cuts + 1 {
            let j: T = T::from_usize(i).unwrap();
            let rect = Rect::<T,T>::new(x + w * p * j, y + h * q * j, w, h);
            let index_b = self.rects.len();
            self.rects.push(TreeRect::new(rect, Some(index)));
            self.rects[index].children.push(index_b);
            self.inherit_edges(index_b, transverse);

            if i > 0 {
                let index_a = index_b - 1;
                let line = Line {
                    start:  Vec2::new(x + w * j * p,            y + h * j * q),
                    end:    Vec2::new(x + w * j * p + w * q,    y + h * j * q + h * p),
                };

                let edge_index = self.edges.len();
                self.edges.push(TreeEdge {
                    a: index_a,
                    b: index_b,
                    line: line.clone(),
                    axis,
                    child_count: 0,
                });

                println!("Edge {} ({} -> {}): {:?} -> {:?}", edge_index, index_a, index_b, line.start, line.end);

                self.rects[index_a].edges.push(EdgeRef(edge_index, transverse));
                self.rects[index_b].edges.push(EdgeRef(edge_index, transverse.opposite()));
            }
        }

        new_rects
    }

    /// Allows a newly formed rect to inherit a parents edges
    fn inherit_edges(&mut self, index: usize, applied_edge: Transverse) {
        if let Some(parent_index) = self.rects[index].parent {
            println!("R{}'s parent is R{}: inheriting {} edges...", index, parent_index, self.rects[parent_index].edges.len());
            let mut edges_to_add = Vec::<(usize, EdgeRef)>::new();

            for EdgeRef(edge_index, transverse) in self.rects[parent_index].edges.iter() {
                println!("  Checking E{}...", edge_index);

                // TODO: Add check to see if this edge even touches our new rect!

                if *transverse != applied_edge && self.edges[*edge_index].active() {
                    let new_edge_index = self.edges.len();
                    let mut new_edge = self.edges[*edge_index].clone();
                    new_edge.child_count = 0;
                    self.edges[*edge_index].child_count += 1;

                    println!("    E{}: cloning and modifying as E{} ({:?})...", edge_index, self.edges.len(), transverse);
                    let other;

                    if new_edge.a == parent_index {
                        new_edge.a = index;
                        other = new_edge.b;
                    } else if new_edge.b == parent_index {
                        new_edge.b = index;
                        other = new_edge.a;
                    } else {
                        panic!("Edge indexed does not belong to parent- this is bad");
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
                            let y2 = a_y2.min(b_y2).into_inner();
                            (x, x, y1, y2)
                        },
                        Transverse::Right => {
                            let x = self.rects[new_edge.a].rect.x + self.rects[new_edge.a].rect.w;
                            let y1 = a_y1.max(b_y1).into_inner();
                            let y2 = a_y2.min(b_y2).into_inner();
                            (x, x, y1, y2)
                        },
                        _ => {
                            panic!("Transverse for anything other than udlr is wrong and you shouldnt do it");
                        }
                    };

                    println!("    Made E{}: x1: {}, x2: {}, y1: {}, y2: {}, a: R{}, b: R{}", new_edge_index, x1, x2, y1, y2, new_edge.a, new_edge.b);

                    new_edge.line = Line {
                        start: Vec2::new(x1, y1),
                        end: Vec2::new(x2, y2),
                    };
                    self.edges.push(new_edge);
                    edges_to_add.push((index, EdgeRef(new_edge_index, transverse.clone())));
                    edges_to_add.push((other, EdgeRef(new_edge_index, transverse.opposite())));
                }
            }
            for (i, eref) in edges_to_add {
                self.rects[i].edges.push(eref);
                println!("    Adding E{} to R{}....", eref.0, i);
            }
        }
    }
}

impl<T> Draw<T> for Treemap<T> where T: OrdNum + Float {
    fn draw(&self, image: &mut ImageBuffer<RgbRaw<u8>, Vec<u8>>, _colour: Rgb<u8>) {

        let font = load_font("assets/DejaVuSans.ttf").unwrap();
        let mut rng = rand::thread_rng();

        for (i, e ) in self.edges.iter().enumerate() {
            if e.active() {
                let col = crate::render::random_colour(&mut rng);
                let text = format!("E{}", i);
                let center = (e.line.start + e.line.end) / (T::one() + T::one());
                e.line.draw(image, col);
                draw_text(image, center.x, center.y, &text, col, &font);
                //println!("E{}: {} -> {}", i, e.line.start, e.line.end);
            }
        }

        for (i, r) in self.rects.iter().enumerate() {
            if r.active() {
                let text = format!("R{}", i);
                let center = r.rect.center();
                let col = if r.active() {Rgb::<u8>::green()} else {Rgb::<u8>::red()};
                draw_text(image,center.x, center.y, &text, col, &font);
                println!("R{} edges: {:?}", i, r.edges);
            }
        }
        println!("there are {} edges", self.edges.len());
        println!("there are {} rects", self.rects.len());
    }
}

#[test]
fn treemap_test() {
    let mut tree_map = Treemap::<f32>::new(Rect::new(0., 0., 510., 510.));
    tree_map.intersect_point(0, Axis::Horizontal, 0.25);
    tree_map.intersect_point(2, Axis::Vertical, 0.5);
    tree_map.intersect_point(3, Axis::Vertical, 0.5);

    println!("---------------------1");

    tree_map.intersect_point(1, Axis::Vertical, 0.1);
    //tree_map.split(3, Axis::Vertical, 2);
    //tree_map.split(2, Axis::Vertical, 3);


    let mut img = ImageBuffer::new(512, 512);
    tree_map.draw(&mut img, Rgb::red());
    img.save("tree_test_2.png").unwrap();
}