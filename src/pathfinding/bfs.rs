use crate::{core::OrdNum, graphs::{Graph, GraphItem}};

use super::Path;

pub struct BreadthFirstSeach<'a, G, T, C, E, N> where T: OrdNum, G: Graph<T, C, E, N>, {
    path: &'a Path<'a, G, T, C, E, N>,
    //position: GraphItem<'a, T, C, E, N>,
}

impl<'a, G, T, C, E, N> BreadthFirstSeach<'a, G, T, C, E, N> where T: OrdNum, G: Graph<T, C, E, N> {
    pub fn new(path: &'a Path<G, T, C, E, N>) -> Self {
        Self {
            path,
        }
    }
}