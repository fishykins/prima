use crate::{core::IndexType, graphs::GraphIndex};

#[derive(Clone, Copy, Debug)]
pub struct Step<Ix>
where
    Ix: IndexType,
{
    pub start: GraphIndex<Ix>,
    pub end: GraphIndex<Ix>,
    pub last: Option<Ix>,
    pub next: Option<Ix>,
}

impl<Ix> Step<Ix>
where
    Ix: IndexType,
{
    pub fn new(start: GraphIndex<Ix>, end: GraphIndex<Ix>) -> Self {
        Self {
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

impl<Ix> std::fmt::Display for Step<Ix>
where
    Ix: IndexType,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} -> {}]", self.start, self.end)
    }
}

impl<Ix> PartialEq for Step<Ix>
where
    Ix: IndexType,
{
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

#[cfg(test)]
mod tests {
    use super::{Step};
    use crate::graphs::{CellIndex, EdgeIndex, NodeIndex};

    #[test]
    fn step_test() {
        let step_a = Step::<usize>::new(CellIndex::new_graph_index(0), EdgeIndex::new_graph_index(1));
        let step_b = Step::<usize>::new(EdgeIndex::new_graph_index(0), NodeIndex::new_graph_index(1));
        let step_c = step_a.reverse();
        assert_eq!(step_a, step_a.clone());
        assert_ne!(step_a, step_b);
        assert_ne!(step_a, step_c);
    }
}