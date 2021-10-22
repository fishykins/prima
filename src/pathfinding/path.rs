use crate::{core::IndexType, graphs::GraphIndex};

#[derive(Clone, Copy, Debug)]
pub struct Path<Ix>
where
    Ix: IndexType,
{
    pub start: GraphIndex<Ix>,
    pub end: GraphIndex<Ix>,
    pub using_nodes: bool,
    pub using_edges: bool,
    pub using_cells: bool,
    pub last: Option<Ix>,
    pub next: Option<Ix>,
}

impl<Ix> Path<Ix>
where
    Ix: IndexType,
{
    pub fn new(start: GraphIndex<Ix>, end: GraphIndex<Ix>) -> Self {
        let mut using_nodes: bool = false;
        let mut using_edges: bool = false;
        let mut using_cells: bool = false;

        match start {
            GraphIndex::Cell(_) => {
                using_cells = true;
            }
            GraphIndex::Edge(_) => {
                using_edges = true;
            }
            GraphIndex::Node(_) => {
                using_nodes = true;
            }
            _ => panic!("No start position provided for path!"),
        }
        match end {
            GraphIndex::Cell(_) => {
                using_cells = true;
            }
            GraphIndex::Edge(_) => {
                using_edges = true;
            }
            GraphIndex::Node(_) => {
                using_nodes = true;
            }
            _ => panic!("No target position provided for path!"),
        }

        Self {
            using_nodes,
            using_edges,
            using_cells,
            start,
            end,
            last: None,
            next: None,
        }
    }

    pub fn reverse(self) -> Self {
        let mut new_self = self.clone();
        new_self.end = self.start;
        new_self.start = self.end;
        new_self
    }

    pub fn extend(start: GraphIndex<Ix>, end: GraphIndex<Ix>, last: Option<Ix>) -> Self {
        let mut new = Self::new(start, end);
        new.last = last;
        new
    }
}

impl<Ix> std::fmt::Display for Path<Ix>
where
    Ix: IndexType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} -> {}]", self.start, self.end)
    }
}

impl<Ix> PartialEq for Path<Ix>
where
    Ix: IndexType,
{
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

#[cfg(test)]
mod tests {
    use super::{Path};
    use crate::graphs::{CellIndex, EdgeIndex, NodeIndex};

    #[test]
    fn path_test() {
        let path_a = Path::<usize>::new(CellIndex::new_graph_index(0), EdgeIndex::new_graph_index(1));
        let path_b = Path::<usize>::new(EdgeIndex::new_graph_index(0), NodeIndex::new_graph_index(1));
        let path_c = path_a.reverse();
        assert_eq!(path_a, path_a.clone());
        assert_ne!(path_a, path_b);
        assert_ne!(path_a, path_c);
    }
}