use super::{TreeEdge, TreeRect};
use crate::core::OrdNum;
use crate::geom::Transverse;
use crate::render::{draw_text, load_font_from_bytes, Draw, ImageBuffer, RgbRaw};
use num::Float;
use vek::Rgb;

pub struct Treemap<T>
where
    T: OrdNum + Float,
{
    pub(crate) rects: Vec<TreeRect<T>>,
    pub(crate) edges: Vec<TreeEdge<T>>,
}

impl<T> Treemap<T>
where
    T: OrdNum + Float,
{
    // Getter for the rect of given index.
    pub fn rect(&self, index: usize) -> Option<&TreeRect<T>> {
        if index >= self.rects.len() {
            return None;
        }
        return Some(&self.rects[index]);
    }

    // Getter for the edge of given index.
    pub fn edge(&self, index: usize) -> Option<&TreeEdge<T>> {
        if index >= self.edges.len() {
            return None;
        }
        return Some(&self.edges[index]);
    }

    // Gets the edges assosiated with the given rect index.
    pub fn rect_edges(&self, rect_index: usize) -> Vec<(usize, &TreeEdge<T>, Transverse)> {
        if rect_index >= self.rects.len() {
            return Vec::new();
        }
        let rect = &self.rects[rect_index];
        return rect
            .edges
            .iter()
            .map(|x| (x.0, &self.edges[x.0], x.1))
            .collect();
    }

    // Gets the neighbor rect that is joined via the given edge.
    pub fn rect_neighbor(
        &self,
        rect_index: usize,
        edge_index: usize,
    ) -> Option<(usize, &TreeRect<T>)> {
        if let Some(edge) = self.edge(edge_index) {
            if let Some(other) = edge.other(rect_index) {
                let rect = self.rect(other).unwrap();
                return Some((other, rect));
            }
        };
        None
    }

    // Gets all neighbor rects for the given rect index.
    pub fn rect_neighbors(&self, rect_index: usize) -> Vec<(usize, &TreeRect<T>)> {
        let mut neighbors = Vec::new();
        if self.rect(rect_index).is_some() {
            let edges = self.rect_edges(rect_index);
            for e in edges.iter() {
                if let Some(n) = self.rect_neighbor(rect_index, e.0) {
                    neighbors.push(n);
                }
            }
        }
        return neighbors;
    }
}

impl<T> Draw<T> for Treemap<T>
where
    T: OrdNum + Float,
{
    // Renders the graph to an image buffer.
    fn draw(&self, image: &mut ImageBuffer<RgbRaw<u8>, Vec<u8>>, _colour: Rgb<u8>) {
        let deja_vu_sans = include_bytes!("../../../assets/DejaVuSans.ttf");
        let font = load_font_from_bytes(deja_vu_sans).unwrap();
        let mut rng = rand::thread_rng();

        for e in self.edges.iter() {
            let col = crate::render::random_colour(&mut rng);
            e.line.draw(image, col);
            //let center = (e.line.start + e.line.end) / (T::one() + T::one());
            //draw_text(image, center.x, center.y, &format!("E{}", i), col, &font);
        }

        for (i, r) in self.rects.iter().enumerate() {
            let text = format!("R{}", i);
            let center = r.rect.center();
            let col = if r.active() {
                Rgb::<u8>::green()
            } else {
                Rgb::<u8>::red()
            };
            draw_text(image, center.x, center.y, &text, col, &font);
        }
    }
}

#[test]
fn treemap_test() {
    use crate::geom::Axis;
    use crate::render::{Draw, ImageBuffer};
    use vek::{Rect, Rgb};

    let mut builder = super::TreemapBuilder::<f32>::new(Rect::new(0., 0., 510., 510.));
    let mut img = ImageBuffer::new(512, 512);

    builder.intersect_point(0, Axis::Horizontal, 0.25);
    builder.split(1, Axis::Vertical, 2);
    builder.intersect_point(2, Axis::Vertical, 0.75);
    builder.split(6, Axis::Horizontal, 2);
    builder.intersect_point(8, Axis::Vertical, 0.75);

    let map: Treemap<f32> = builder.build();

    map.draw(&mut img, Rgb::red());
    img.save("tree_test.png").unwrap();
}
