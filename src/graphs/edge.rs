use crate::base::Index;

/// An edge that connects two indexable entities, as referenced by the given index type.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edge<Ix>
where
    Ix: Index,
{
    /// The index of the first point.
    pub a: Ix,
    /// The index of the second point.
    pub b: Ix,
    /// Is this edge directional?
    pub directed: bool,
}

/// A double linked edge, used for graphs that have linked cells and vertices.
pub struct EdgePair<I, J>
where
    I: Index,
    J: Index,
{
    /// The index of the first edge.
    pub a: Edge<I>,
    /// The index of the second edge.
    pub b: Edge<J>,
}

impl<I> Edge<I>
where
    I: Index,
{
    /// Creates a new undirected edge.
    pub fn new(a: I, b: I) -> Self {
        Edge {
            a,
            b,
            directed: false,
        }
    }

    /// Creates a new directed edge.
    pub fn directed(a: I, b: I) -> Edge<I> {
        Edge {
            a,
            b,
            directed: true,
        }
    }

    /// Returns the index of the other vertex.
    pub fn other(&self, index: I) -> Option<I> {
        if self.a == index {
            Some(self.b)
        } else if self.b == index {
            Some(self.a)
        } else {
            None
        }
    }
}

impl<I, J> EdgePair<I, J>
where
    I: Index,
    J: Index,
{
    /// Creates a new edge pair.
    pub fn new(a: Edge<I>, b: Edge<J>) -> EdgePair<I, J> {
        EdgePair { a, b }
    }
}
