use crate::{
    core::{IndexType, OrdNum},
    graphs::Graph,
};

use super::Path;

pub struct BreadthFirstSeach<'a, G, T, C, E, N, Ix>
where
    T: OrdNum,
    G: Graph<T, C, E, N, Ix>,
    Ix: IndexType,
{
    path: &'a Path<'a, G, T, C, E, N, Ix>,
    //position: GraphItem<'a, T, C, E, N>,
}

impl<'a, G, T, C, E, N, Ix> BreadthFirstSeach<'a, G, T, C, E, N, Ix>
where
    T: OrdNum,
    G: Graph<T, C, E, N, Ix>,
    Ix: IndexType,
{
    pub fn new(path: &'a Path<G, T, C, E, N, Ix>) -> Self {
        Self { path }
    }
}
