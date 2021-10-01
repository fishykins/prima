use crate::core::{DefaultIx, OrdNum};
use vek::Vec2;

pub struct Node<T, N, Ix = DefaultIx>
where
    T: OrdNum,
{
    x: T,
    y: T,
    data: N,
    linked_nodes: Vec<Ix>,
    linked_edges: Vec<Ix>,
    linked_cells: Vec<Ix>,
}

impl<T, N, Ix> Node<T, N, Ix>
where
    T: OrdNum,
{
    pub fn new(
        x: T,
        y: T,
        linked_nodes: Vec<Ix>,
        linked_edges: Vec<Ix>,
        linked_cells: Vec<Ix>,
        data: N,
    ) -> Self {
        Self {
            x,
            y,
            linked_cells,
            linked_edges,
            linked_nodes,
            data,
        }
    }

    pub fn pos(&self) -> Vec2<T> {
        Vec2::new(self.x, self.y)
    }

    pub fn linked_nodes(&self) -> &Vec<Ix> {
        &self.linked_nodes
    }

    pub fn linked_cells(&self) -> &Vec<Ix> {
        &self.linked_cells
    }

    pub fn linked_edges(&self) -> &Vec<Ix> {
        &self.linked_edges
    }

    pub fn data(&self) -> &N {
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut N {
        &mut self.data
    }
}
