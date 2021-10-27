use crate::core::{DefaultIx, IndexType};

macro_rules! index {
    ($index_type:ident, $graphindex_name:ident) => {
        #[derive(Copy, Clone, Debug, PartialOrd, Ord, Eq, Hash, Default)]
        pub struct $index_type<Ix = DefaultIx>(pub Ix)
        where
            Ix: IndexType;

        impl<Ix> PartialEq for $index_type<Ix>
        where
            Ix: IndexType,
        {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl<Ix> $index_type<Ix> where Ix: IndexType {
            #[inline(always)]
            pub fn new(x: usize) -> Self {
                Self(Ix::new(x))
            }
            #[inline(always)]
            pub fn index(&self) -> usize {
                self.0.index()
            }
            #[inline(always)]
            pub fn graph_index(self) -> GraphIndex<Ix> {
                GraphIndex::$graphindex_name(self)
            }
            #[inline(always)]
            pub fn newi(x: usize) -> GraphIndex<Ix> {
                GraphIndex::$graphindex_name(Self::new(x))
            }
        }
    };
}

index!(EdgeIndex, Edge);
index!(CellIndex, Cell);
index!(NodeIndex, Node);
index!(StepIndex, Step);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GraphIndex<Ix> where Ix : IndexType {
    Cell(CellIndex<Ix>),
    Edge(EdgeIndex<Ix>),
    Node(NodeIndex<Ix>),
    Step(StepIndex<Ix>),
    None,
}

impl<Ix> std::fmt::Display for GraphIndex<Ix> where Ix: IndexType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GraphIndex::Cell(i) => write!(f, "Cell {}", i.index()),
            GraphIndex::Edge(i) => write!(f, "Edge {}", i.index()),
            GraphIndex::Node(i) => write!(f, "Node {}", i.index()),
            GraphIndex::Step(i) => write!(f, "Step {}", i.index()),
            GraphIndex::None => todo!(),
        }
        
    }
}