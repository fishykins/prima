use crate::core::{DefaultIx, IndexType, OrdNum};
use super::Node;

#[derive(Copy, Clone)]
pub struct NodeRef<'a, T, N, Ix = DefaultIx>(pub Ix, pub &'a Node<T, N, Ix>)
where
    T: OrdNum,
    Ix: IndexType;


impl<'a, T, N, Ix> NodeRef<'a, T, N, Ix>
where
    T: OrdNum,
    Ix: IndexType,
{
    pub fn new(index: Ix, node: &'a Node<T, N, Ix>) -> Self {
        Self(index, node)
    }
}

