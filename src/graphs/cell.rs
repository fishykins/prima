use crate::core::{DefaultIx, IndexType};

pub struct Cell<C, Ix = DefaultIx>
where
    Ix: IndexType,
{
    pub(crate) edges: Vec<Ix>,
    pub data: Option<Box<C>>,
}

impl<C, Ix> Cell<C, Ix>
where
    Ix: IndexType,
{
    pub fn new(edges: Vec<Ix>, data: Option<Box<C>>) -> Self {
        Self { edges, data }
    }

    pub fn data(&self) -> Option<&Box<C>> {
        if self.data.is_none() {
            return None;
        }
        self.data.as_ref()
    }

    pub fn edges(&self) -> Vec<Ix> {
        self.edges.clone()
    }
}

impl<C, Ix> PartialEq for Cell<C, Ix>
where
    Ix: IndexType,
{
    fn eq(&self, other: &Self) -> bool {
        self.edges == other.edges
    }
}

#[cfg(test)]
mod tests {
    use crate::graphs::Cell;

    #[test]
    fn cell_test() {
        let cell = Cell::new(vec![0u32,1u32,2u32], Some(Box::new("32")));
        let cell_box = cell.data.unwrap();
        let _clone_cell_box = cell_box.clone();
    }
}