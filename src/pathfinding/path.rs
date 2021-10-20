use std::marker::PhantomData;

use crate::{core::{IndexType, OrdNum}, graphs::{Graph, GraphIndex}};

pub struct Path<'a, G, T, C, E, N, Ix>
where
    T: OrdNum,
    G: Graph<T, C, E, N, Ix>,
    Ix: IndexType
{
    pub graph: &'a G,
    pub start: GraphIndex<Ix>,
    pub target: GraphIndex<Ix>,
    pub using_nodes: bool,
    pub using_edges: bool,
    pub using_cells: bool,
    p_num: PhantomData<T>,
    p_cell_data: PhantomData<C>,
    p_edge_data: PhantomData<E>,
    p_node_data: PhantomData<N>,
}

impl<'a, G, T, C, E, N, Ix> Path<'a, G, T, C, E, N, Ix>
where
    T: OrdNum,
    G: Graph<T, C, E, N, Ix>,
    Ix: IndexType
{
    pub fn new(graph: &'a G, start: GraphIndex<Ix>, target: GraphIndex<Ix>) -> Self {

        let mut using_nodes: bool = false;
        let mut using_edges: bool = false;
        let mut using_cells: bool = false;

        match start {
            GraphIndex::Cell(_) => {
                using_cells = true;
            },
            GraphIndex::Edge(_) => {
                using_edges = true;
            },
            GraphIndex::Node(_) => {
                using_nodes = true;
            },
        }
        match target {
            GraphIndex::Cell(_) => {
                using_cells = true;
            },
            GraphIndex::Edge(_) => {
                using_edges = true;
            },
            GraphIndex::Node(_) => {
                using_nodes = true;
            },
        }

        Self {
            graph,
            using_nodes,
            using_edges,
            using_cells,
            start,
            target,
            p_num: PhantomData,
            p_cell_data: PhantomData,
            p_edge_data: PhantomData,
            p_node_data: PhantomData,
        }
    }
}
