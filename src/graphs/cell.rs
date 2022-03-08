/// A cell within a graph.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Cell<D, Ix> {
    /// Vector of indices of the edges that are connected to this cell.
    pub edges: Vec<Ix>,
    /// Generic data stored in the cell.
    pub data: Option<D>,
}