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

    // Finds the index of the given rect.
    pub fn rect_index(&self, rect: &TreeRect<T>) -> usize {
        return self.rects.iter().position(|x| x == rect).unwrap();
    }

    // Finds the index of the given edge.
    pub fn edge_index(&self, edge: &TreeEdge<T>) -> usize {
        return self.edges.iter().position(|x| x == edge).unwrap();
    }

    // Counts the total number of rects in the graph.
    pub fn rect_count(&self) -> usize {
        self.rects.len()
    }

    // Counts the total number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    // Returns a refference to every rect in the graph.
    pub fn rects(&self) -> Vec<&TreeRect<T>> {
        return self.rects.iter().map(|x| x).collect();
    }

    // Returns a refference to every edge in the graph.
    pub fn edges(&self) -> Vec<&TreeEdge<T>> {
        return self.edges.iter().map(|x| x).collect();
    }

    // Gets the edges assosiated with the given rect index.
    pub fn rect_edges(&self, rect: &TreeRect<T>) -> Vec<(usize, &TreeEdge<T>, Transverse)> {
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
    pub fn rect_neighbors(&self, rect: &TreeRect<T>) -> Vec<(usize, &TreeRect<T>)> {
        let mut neighbors = Vec::new();
        let edges = self.rect_edges(rect);
        for e in edges.iter() {
            if let Some(n) = self.rect_neighbor(self.rect_index(rect), e.0) {
                neighbors.push(n);
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
