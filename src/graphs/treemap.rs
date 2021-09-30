use crate::geom::{Axis, Rect, Line, Transverse};
use crate::render::{Draw, draw_text, ImageBuffer, RgbRaw, load_font_from_bytes};
use crate::core::{OrdNum, maths::clamp01};
use num::{Float};
use std::{fmt::Debug};
use vek::{Vec2, Rgb};
use ordered_float::OrderedFloat;

#[derive(Debug, Copy, Clone)]
pub struct EdgeRef(usize, Transverse);

#[derive(Debug, Clone)]
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
    birth_cycle: Option<usize>,
}

pub struct Treemap<T> where T: OrdNum + Float {

    rects: Vec<TreeRect<T>>,
    edges: Vec<TreeEdge<T>>,
    cycle: usize,
}

impl<T> TreeEdge<T> where T: OrdNum + Float {
    pub fn active(&self) -> bool {
        self.birth_cycle.is_none()
    }

    pub fn a(&self) -> usize {
        self.a.clone()
    }

    pub fn b(&self) -> usize {
        self.b.clone()
    }

    pub fn other(&self, index: usize) -> Option<usize> {
        if self.a != index && self.b != index {
            return None;
        }
        if self.a == index {
            return Some(self.b);
        }
        Some(self.a)
    }

    fn can_split(&self, current_cycle: usize) -> bool {
        if self.birth_cycle.is_some() {
            self.birth_cycle.unwrap() == current_cycle
        } else {
            true
        }
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

    pub fn active(&self) -> bool {
        self.children.len() == 0
    }
}

impl<T> Treemap<T> where T: OrdNum + Float {
    pub fn new(rect: Rect<T, T>) -> Self {
        Self {
            rects: vec![TreeRect::new(rect, None)],
            edges: Vec::new(),
            cycle: 0,
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

    /// Getter for all active rectangles
    pub fn rects(&self) -> Vec<usize> {
        self.rects.iter().enumerate().filter(|(_, x)| x.active()).map(|(i,_)| i).collect()
    }

    /// Getter for all active edges
    pub fn edges(&self) -> Vec<usize> {
        self.edges.iter().enumerate().filter(|(_, x)| x.active()).map(|(i,_)| i).collect()
    }

    pub fn rect_edges(&self, rect_index: usize) -> Vec<usize> {
        if rect_index >= self.rects.len() {
            return Vec::new();
        }
        let rect = &self.rects[rect_index];
        rect.edges.iter().filter(|x| self.edges[x.0].active() ).map(|x| x.0).collect()
    }

    pub fn intersect_point(&mut self, index: usize, axis: Axis, offset: T) -> bool {
        if index >= self.rects.len() {
            panic!("Index {} falls outside the range of {}: intersection will fail", index, self.rects.len());
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
            birth_cycle: None,
        });

        self.rects.push(TreeRect::new(rect_a, Some(index)));
        self.rects.push(TreeRect::new(rect_b, Some(index)));
        self.rects[index].children.push(index_a);
        self.rects[index].children.push(index_b);
        self.rects[index_a].edges.push(EdgeRef(edge_index, transverse));
        self.rects[index_b].edges.push(EdgeRef(edge_index, transverse.opposite()));
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

        let new_rects= Vec::new();

        let n = T::from_usize(cuts + 1).unwrap();
        let x = self.rects[index].rect.x;
        let y = self.rects[index].rect.y;

        let (w, h, p, q, transverse) = match axis {
            Axis::Horizontal => (self.rects[index].rect.w, self.rects[index].rect.h / n, T::zero(), T::one(), Transverse::Up),
            Axis::Vertical => (self.rects[index].rect.w  / n, self.rects[index].rect.h, T::one(), T::zero(), Transverse::Right),
            _ => {panic!("No axis")},
        };

        for i in 0..cuts + 1 {
            let j: T = T::from_usize(i).unwrap();
            let rect = Rect::<T,T>::new(x + w * p * j, y + h * q * j, w, h);
            let index_b = self.rects.len();
            self.rects.push(TreeRect::new(rect, Some(index)));
            self.rects[index].children.push(index_b);
            
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
                    birth_cycle: None,
                });
                
                println!("Edge {} ({} -> {}): {:?} -> {:?}", edge_index, index_a, index_b, line.start, line.end);
                
                self.rects[index_a].edges.push(EdgeRef(edge_index, transverse));
                self.rects[index_b].edges.push(EdgeRef(edge_index, transverse.opposite()));
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
        println!("R{} is inheriting, using applied edge {:?} (cycle {})", rect_index, applied_edge, self.cycle);
        if let Some(parent_index) = self.rects[rect_index].parent {
            println!("R{}'s parent is R{}: inheriting {} edges...", rect_index, parent_index, self.rects[parent_index].edges.len());
            let mut edges_to_add = Vec::<(usize, EdgeRef)>::new();

            for EdgeRef(edge_index, transverse) in self.rects[parent_index].edges.iter() {
                println!("  Checking E{}...", edge_index);

                if *transverse != applied_edge && self.edges[*edge_index].can_split(self.cycle) {
                    let new_edge_index = self.edges.len();
                    let mut new_edge = self.edges[*edge_index].clone();

                    println!("    E{}: cloning as E{} ({:?})...", edge_index, self.edges.len(), transverse);
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
                    let a_x2 = OrderedFloat(self.rects[new_edge.a].rect.x + self.rects[new_edge.a].rect.w);
                    let b_x2 = OrderedFloat(self.rects[new_edge.b].rect.x + self.rects[new_edge.b].rect.w);

                    let a_y1 = OrderedFloat(self.rects[new_edge.a].rect.y);
                    let b_y1 = OrderedFloat(self.rects[new_edge.b].rect.y);
                    let a_y2 = OrderedFloat(self.rects[new_edge.a].rect.y + self.rects[new_edge.a].rect.h);
                    let b_y2 = OrderedFloat(self.rects[new_edge.b].rect.y + self.rects[new_edge.b].rect.h);

                    let valid: bool;

                    let (x1, x2, y1, y2) = match transverse {
                        Transverse::Up => {
                            let x1 = Ord::max(a_x1, b_x1).into_inner();
                            let x2 = Ord::min(a_x2, b_x2).into_inner();
                            let y = self.rects[new_edge.b].rect.y;// + self.rects[new_edge.b].rect.h;
                            valid = self.validate_horizontal(new_edge.a, new_edge.b);
                            (x1, x2, y, y)
                        },
                        Transverse::Down => {
                            let x1 = Ord::max(a_x1, b_x1).into_inner();
                            let x2 = Ord::min(a_x2, b_x2).into_inner();
                            let y = self.rects[new_edge.b].rect.y;
                            valid = self.validate_horizontal(new_edge.a, new_edge.b);
                            (x1, x2, y, y)
                        },
                        Transverse::Left => {
                            let x = self.rects[new_edge.b].rect.x;
                            let y1 = Ord::max(a_y1, b_y1).into_inner();
                            let y2 = Ord::min(a_y2, b_y2).into_inner();
                            valid = self.validate_vertical(new_edge.a, new_edge.b);
                            (x, x, y1, y2)
                        },
                        Transverse::Right => {
                            let x = self.rects[new_edge.b].rect.x;
                            let y1 = Ord::max(a_y1, b_y1).into_inner();
                            let y2 = Ord::min(a_y2, b_y2).into_inner();
                            valid = self.validate_vertical(new_edge.a, new_edge.b);
                            (x, x, y1, y2)
                        },
                        _ => {
                            panic!("Transverse for anything other than udlr is wrong and you shouldnt do it");
                        }
                    };

                    if valid {
                        new_edge.line = Line {
                            start: Vec2::new(x1, y1),
                            end: Vec2::new(x2, y2),
                        };
                        println!("    Made E{}: x1: {}, x2: {}, y1: {}, y2: {}, a: R{}, b: R{}", new_edge_index, x1, x2, y1, y2, new_edge.a, new_edge.b);
                        self.edges.push(new_edge);
                        edges_to_add.push((rect_index, EdgeRef(new_edge_index, transverse.clone())));
                        edges_to_add.push((other, EdgeRef(new_edge_index, transverse.opposite())));
                        self.edges[*edge_index].birth_cycle = Some(self.cycle);
                        self.edges[new_edge_index].birth_cycle = None;

                    } else {
                        println!("    Could not make E{}- does not lie on an adjacant to R{}", new_edge_index, rect_index);
                        println!("        a: [x = {}, x2 = {}, y = {}, y2 = {}]", self.rects[new_edge.a].rect.x, self.rects[new_edge.a].rect.x + self.rects[new_edge.a].rect.w, self.rects[new_edge.a].rect.y, self.rects[new_edge.a].rect.y + self.rects[new_edge.a].rect.h);
                        println!("        b: [x = {}, x2 = {}, y = {}, y2 = {}]", self.rects[new_edge.b].rect.x, self.rects[new_edge.b].rect.x + self.rects[new_edge.b].rect.w, self.rects[new_edge.b].rect.y, self.rects[new_edge.b].rect.y + self.rects[new_edge.b].rect.h);
                    }
                }
            }
            for (i, eref) in edges_to_add {
                self.rects[i].edges.push(eref);
                println!("    Adding E{} to R{}....", eref.0, i);
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

        (a_in_b || b_in_a || a_overlaps_left || a_overlaps_right) && (rect_a.y == rect_b_y2 || rect_a_y2 == rect_b.y)
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

        (a_in_b || b_in_a || a_overlaps_down || a_overlaps_up) && (rect_a.x == rect_b_x2 || rect_a_x2 == rect_b.x)
    }
}

impl<T> Draw<T> for Treemap<T> where T: OrdNum + Float {
    fn draw(&self, image: &mut ImageBuffer<RgbRaw<u8>, Vec<u8>>, _colour: Rgb<u8>) {
        let deja_vu_sans = include_bytes!("../../assets/DejaVuSans.ttf");
        let font = load_font_from_bytes(deja_vu_sans).unwrap();
        let mut rng = rand::thread_rng();

        for (_, e ) in self.edges.iter().enumerate() {
            if e.active() {
                let col = crate::render::random_colour(&mut rng);
                e.line.draw(image, col);
                //let center = (e.line.start + e.line.end) / (T::one() + T::one());
                //draw_text(image, center.x, center.y, &format!("E{}", i), col, &font);
            }
        }

        for (i, r) in self.rects.iter().enumerate() {
            if r.active() {
                let text = format!("R{}", i);
                let center = r.rect.center();
                let col = if r.active() {Rgb::<u8>::green()} else {Rgb::<u8>::red()};
                draw_text(image,center.x, center.y, &text, col, &font);
            }
        }
    }
}

#[test]
fn treemap_test() {
    let mut tree_map = Treemap::<f32>::new(Rect::new(0., 0., 510., 510.));
    let mut img1 = ImageBuffer::new(512, 512);
    let mut img2 = ImageBuffer::new(512, 512);

    tree_map.intersect_point(0, Axis::Horizontal, 0.25);
    tree_map.split(1, Axis::Vertical, 2);
    tree_map.intersect_point(2, Axis::Vertical, 0.75);
    tree_map.split(6, Axis::Horizontal, 2);

    
    tree_map.draw(&mut img1, Rgb::red());
    println!("---------------------");

    tree_map.intersect_point(8, Axis::Vertical, 0.75);
    tree_map.draw(&mut img2, Rgb::red());
    img1.save("tree_test.png").unwrap();
    img2.save("tree_test_2.png").unwrap();

    println!("there are a total of {} rects and {} edges", tree_map.rects().len(), tree_map.edges().len());
    let index = 12;
    println!("R{} has edges {:?}", index, tree_map.rect_edges(index));
}