use crate::base::DefaultIndex;

/// A vertex represents a point in a graph-like structure. 
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Vertex<D, P, Ix = DefaultIndex> {
    /// The position of the vertex.
    pub point: P,
    /// The indices of the connected edges.
    pub edges: Vec<Ix>,
    /// Data stored in the vertex.
    pub data: Option<D>,
}