use crate::core::{DefaultIx, IndexType};
use super::Edge;


#[derive(Copy, Clone)]
pub struct EdgeRef<'a, E, Ix = DefaultIx>(pub Ix, pub &'a Edge<E, Ix>)
where
    Ix: IndexType;


impl<'a, E, Ix> EdgeRef<'a, E, Ix>
where
    Ix: IndexType,
{
    pub fn new(index: Ix, edge: &'a Edge<E, Ix>) -> Self {
        Self(index, edge)
    }
}