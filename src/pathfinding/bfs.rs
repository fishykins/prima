use std::collections::VecDeque;

use crate::{
    core::{IndexType, OrdNum},
    graphs::{Graph, GraphIndex},
};

use super::Step;

pub struct BreadthFirstSeach<'a, T, C, E, N, Ix>
where
    T: OrdNum,
    Ix: IndexType,
{
    graph: &'a dyn Graph<T, C, E, N, Ix>,
    step: Step<Ix>,
    open: VecDeque<Step<Ix>>,
    closed: Vec<Step<Ix>>,
    route: Vec<GraphIndex<Ix>>,
}

impl<'a, T, C, E, N, Ix> BreadthFirstSeach<'a, T, C, E, N, Ix>
where
    T: OrdNum,
    Ix: IndexType,
{
    pub fn new(graph: &'a dyn Graph<T, C, E, N, Ix>, step: Step<Ix>) -> Self {
        let mut me = Self {
            graph,
            step,
            open: VecDeque::new(),
            closed: Vec::new(),
            route: Vec::new(),
        };

        me.steps(Step::<Ix>::new(me.step.start, me.step.start), None);
        return me;
    }

    /// Calculates a single step of the step. Returns None if the step is finished, or has somehow failed.
    /// To retreive the finished route, call [`route`]. [`calculate`] is the more appropriate way to obtain a step in 90% of cases,
    /// this is only needed if you wish to split the stepfinding over several ticks.
    pub fn step(&mut self) -> Option<Step<Ix>> {
        if let Some(next) = self.open.pop_front() {
            println!("Popped open step {}", next);
            let i = self.closed.len();
            self.closed.push(next);
            let found_route = self.steps(next, Some(Ix::new(i)));
            if found_route {
                return None;
            }
            return Some(self.closed[i]);
        }
        return None;
    }

    /// Builds the full step in one swift pass. 
    pub fn calculate(&mut self) -> Vec<GraphIndex<Ix>> {
        while self.step().is_some() {}
        return self.route()
    }

    pub fn route(&self) -> Vec<GraphIndex<Ix>> {
        self.route.clone().into_iter().rev().collect()
    }

    fn build_route(&mut self, step: Step<Ix>) {
        self.route.push(step.end);
        if let Some(last_step_index) = step.last {
            let last_step = self.closed[last_step_index.index()];
            self.build_route(last_step);
        } else {
            self.route.push(step.start);
        }
    }

    // Gets all the steps from the given index and adds them to the open que.
    fn steps(&mut self, next: Step<Ix>, last: Option<Ix>) -> bool {
        let index: GraphIndex<Ix> = next.end;
        match index {
            GraphIndex::Cell(cell_index) => {
                // Cell -> Edges
                for i in self.graph.cell_edges(cell_index) {
                    let x = GraphIndex::Edge(i);
                    let step = Step::extend(index, x, last);
                    if step.end == self.step.end {
                        // We have found it!
                        self.build_route(step);
                        return true;
                    }
                    let step_opposite = Step::new(x, index);
                    if !self.closed.contains(&step)
                        && !self.closed.contains(&step_opposite)
                        && !self.open.contains(&step)
                        && !self.open.contains(&step_opposite)
                        && step_opposite != next
                    {
                        println!("pushing {} to back of open", step);
                        self.open.push_back(step);
                    }
                }
            }
            GraphIndex::Edge(edge_index) => {
                let edge_cells = self.graph.edge_cells(edge_index);
                for i in vec![edge_cells.0, edge_cells.1] {
                    let x = GraphIndex::Cell(i);
                    let step = Step::extend(index, x, last);
                    if step.end == self.step.end {
                        // We have found it!
                        self.build_route(step);
                        return true;
                    }
                    let step_opposite = step.reverse();
                    if !self.closed.contains(&step)
                        && !self.closed.contains(&step_opposite)
                        && !self.open.contains(&step)
                        && !self.open.contains(&step_opposite)
                        && step_opposite != next
                    {
                        println!("pushing {} to back of open", step);
                        self.open.push_back(step);
                    }
                }
            }
            GraphIndex::Node(_index) => todo!(),
            _ => panic!("GraphIndex should never be None!"),
        };
        false
    }
}
#[cfg(test)]
mod tests {
    use vek::Rect;

    use crate::{
        geom::Axis,
        graphs::{tree_graph::*, CellIndex},
        pathfinding::Step,
    };

    use super::BreadthFirstSeach;

    fn build_test_graph() -> TreeGraph<f32, u8, u8, u8> {
        let mut builder = TreeBuilder::<f32>::new(Rect::new(0., 0., 1080., 1080.));
        builder.intersect_point(0, Axis::Horizontal, 0.25);
        builder.split(1, Axis::Vertical, 2);
        builder.intersect_point(2, Axis::Vertical, 0.75);
        builder.split(6, Axis::Horizontal, 2);
        builder.intersect_point(8, Axis::Vertical, 0.75);
        builder.build_graph()
    }

    #[test]
    fn bfs_test() {
        let graph = build_test_graph();
        let step = Step::<usize>::new(CellIndex::new_graph_index(0), CellIndex::new_graph_index(3));
        let mut bfs = BreadthFirstSeach::new(&graph, step);
        let result = bfs.calculate();
        println!("result: {:?}", result);

        let mut image = image::RgbImage::new(1080, 1080);
        crate::render::draw_graph(&mut image, Box::new(&graph), false);
        let _ = image.save("bfs_test.png").unwrap();
    }
}
