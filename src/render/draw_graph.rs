use super::{deja_vu_sans, draw_line_segment, draw_text, RgbImage};
use crate::core::{IndexType, OrdNum};
use crate::graphs::{Graph, EdgeIndex, CellIndex};
use num::Signed;
use vek::LineSegment2;
//use vek::{Rgb, Vec2};

/// draws a generic graph
pub fn draw_graph<T, C, E, N, Ix>(
    image: &mut RgbImage,
    graph: Box<&dyn Graph<T, C, E, N, Ix>>,
    draw_cell_edge_lines: bool,
) where
    C: Copy,
    E: Copy,
    N: Copy,
    T: OrdNum + Signed,
    Ix: IndexType,
{
    let mut rng = rand::thread_rng();
    let font = deja_vu_sans();

    // First, lets draw every edge and label it
    for (i, _) in graph.edges().iter().enumerate() {
        let col = crate::render::random_colour(&mut rng);
        let edge_ref = EdgeIndex::new(i);
        let line = graph.line(edge_ref);
        let center = (line.start + line.end) / (T::one() + T::one());
        draw_line_segment(image, &line, col);
        draw_text(image, center.x, center.y, &format!("E{}", i), col, &font);
    }

    // First, lets draw every edge and label it
    for (i, _) in graph.cells().iter().enumerate() {
        let col = crate::render::random_colour(&mut rng);
        let cell_ref = CellIndex::new(i);
        let cell_center = graph.center(cell_ref);
        draw_text(
            image,
            cell_center.x,
            cell_center.y,
            &format!("C{}", i),
            col,
            &font,
        );
        if draw_cell_edge_lines {
            for edge_ref in graph.cell_edges(cell_ref) {
                let edge_line = graph.line(edge_ref);
                let edge_center = (edge_line.start + edge_line.end) / (T::one() + T::one());
                let link_line = LineSegment2 {
                    start: cell_center,
                    end: edge_center,
                };
                draw_line_segment(image, &link_line, col / 4);
            }
        }
    }
}
