use crate::geom::{Axis, Rect};
use num::{Num};
use std::ops::{DivAssign, AddAssign};

struct TreeNode<T> where T: Num + Copy + DivAssign + AddAssign {
    rect: Rect<T, T>,
    parent: Option<usize>,
    children: Vec<usize>,
}

pub struct Treemap<T> where T: Num + Copy + DivAssign + AddAssign {
    nodes: Vec<TreeNode<T>>,
}


impl<T> TreeNode<T> where T: Num + Copy + DivAssign + AddAssign {
    pub fn new(rect: Rect<T, T>, parent: Option<usize>) -> Self {
        Self {
            rect,
            parent,
            children: Vec::new(),
        }
    }
}

impl<T> Treemap<T> where T: Num + Copy + DivAssign + AddAssign {
    pub fn new(rect: Rect<T, T>) -> Self {
        Self {
            nodes: vec![TreeNode {
                rect,
                parent: None,
                children: Vec::new(),
            }],
        }
    }

    pub fn split(&mut self, index: usize, axis: Axis) -> (usize, usize) {
        let (a, b) = match axis {
            Axis::Horizontal => {
                let x = self.nodes[index].rect.x;
                let w = self.nodes[index].rect.w;
                let y = self.nodes[index].rect.y;
                let h = self.nodes[index].rect.h / (T::one() + T::one());
                let rect_top = Rect::new(x, h, w, h);
                let rect_bottom = Rect::new(x, y, w, h);
                (rect_bottom, rect_top)
            },
            Axis::Vertical => {
                let x = self.nodes[index].rect.x;
                let w = self.nodes[index].rect.w  / (T::one() + T::one());
                let y = self.nodes[index].rect.y;
                let h = self.nodes[index].rect.h;
                let rect_left = Rect::new(x, y, w, h);
                let rect_right = Rect::new(w, y, w, h);
                (rect_left, rect_right)
            },
            _ => {
                panic!();
            }
        };

        let node_a = TreeNode::new(a, Some(index));
        let node_b = TreeNode::new(b, Some(index));

        let index_a = self.nodes.len();
        let index_b = index_a + 1;

        self.nodes[index].children.push(index_a);
        self.nodes[index].children.push(index_b);
        self.nodes.push(node_a);
        self.nodes.push(node_b);

        (index_a, index_b)
    }
}

#[test]
fn treemap_test() {
    let _tree_map = Treemap::<f32>::new(Rect::new(0., 0., 64., 64.));
}