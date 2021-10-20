use std::marker::PhantomData;

use crate::{core::OrdNum, graphs::{Graph, GraphItem}};

pub struct Path<'a, G, T, C, E, N>
where
    T: OrdNum,
    G: Graph<T, C, E, N>,
{
    pub graph: &'a G,
    pub start: GraphItem<'a, T, C, E, N>,
    pub target: GraphItem<'a, T, C, E, N>,
    pub using_nodes: bool,
    pub using_edges: bool,
    pub using_cells: bool,
    p_num: PhantomData<T>,
    p_cell_data: PhantomData<C>,
    p_edge_data: PhantomData<E>,
    p_node_data: PhantomData<N>,
}

impl<'a, G, T, C, E, N> Path<'a, G, T, C, E, N>
where
    T: OrdNum,
    G: Graph<T, C, E, N>,
{
    pub fn new(graph: &'a G, start: GraphItem<'a, T, C, E, N>, target: GraphItem<'a, T, C, E, N>) -> Self {

        let mut using_nodes: bool = false;
        let mut using_edges: bool = false;
        let mut using_cells: bool = false;

        match start {
            GraphItem::Cell(_) => {
                using_cells = true;
            },
            GraphItem::Edge(_) => {
                using_edges = true;
            },
            GraphItem::Node(_) => {
                using_nodes = true;
            },
        }
        match target {
            GraphItem::Cell(_) => {
                using_cells = true;
            },
            GraphItem::Edge(_) => {
                using_edges = true;
            },
            GraphItem::Node(_) => {
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
