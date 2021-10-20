use crate::core::OrdNum;

use super::{CellRef, EdgeRef, NodeRef};

#[derive(Copy, Clone)]
/// A wrapper enum for cells, edges and nodes. 
pub enum GraphItem<'a, T, C, E, N> where T: OrdNum {
    Cell(CellRef<'a, C>),
    Edge(EdgeRef<'a, E>),
    Node(NodeRef<'a, T, N>),
}