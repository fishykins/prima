use crate::core::{DefaultIx, IndexType};
use super::Cell;


#[derive(Copy, Clone)]
pub struct CellRef<'a, C, Ix = DefaultIx>(pub Ix, pub &'a Cell<C, Ix>)
where
    Ix: IndexType;


    
impl<'a, C, Ix> CellRef<'a, C, Ix>
where
    Ix: IndexType,
{
    pub fn new(index: Ix, cell: &'a Cell<C, Ix>) -> Self {
        Self(index, cell)
    }
}