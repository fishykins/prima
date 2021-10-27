use crate::core::{DefaultIx, IndexType};

macro_rules! index {
    ($index_type:ident, $graphindex_name:ident) => {
        /// Unique index type for given graph item.
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
            /// Produces a new index with value x.
            #[inline(always)]
            pub fn new(x: usize) -> Self {
                Self(Ix::new(x))
            }
            /// Retrieves the usize value.
            #[inline(always)]
            pub fn index(&self) -> usize {
                self.0.index()
            }
            /// Converts the index to a [`GraphIndex`], allowing for more generic use.
            #[inline(always)]
            pub fn graph_index(self) -> GraphIndex<Ix> {
                GraphIndex::$graphindex_name(self)
            }
            /// Builds a ['GraphIndex'] directly from the provided index.
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

/// A wrapper enum to hold all 
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GraphIndex<Ix> where Ix : IndexType {
    /// A [`super::Cell`] refference;
    Cell(CellIndex<Ix>),
    /// A [`super::Edge`] refference;
    Edge(EdgeIndex<Ix>),
    /// A [`super::Node`] refference;
    Node(NodeIndex<Ix>),
    /// A [`super::Step`] refference;
    Step(StepIndex<Ix>),
    /// None type for refference.
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