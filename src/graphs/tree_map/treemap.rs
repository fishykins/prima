use super::{TreeEdge, TreeRect};
use crate::core::OrdNum;
use crate::geom::Transverse;
#[cfg(feature = "rendering")]
use crate::render::{deja_vu_sans, draw_text, Draw, ImageBuffer, RgbRaw};
use num::Float;

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

    // Gets the edges assosiated with the given rect index on the given transverse.
    pub fn rect_transverse(
        &self,
        rect: &TreeRect<T>,
        trans: Transverse,
    ) -> Vec<(usize, &TreeEdge<T>)> {
        return rect
            .edges
            .iter()
            .filter(|x| x.1 == trans)
            .map(|x| (x.0, &self.edges[x.0]))
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

#[cfg(feature = "rendering")]
impl<T> Draw<T> for Treemap<T>
where
    T: OrdNum + Float,
{
    // Renders the graph to an image buffer.
    fn draw(&self, image: &mut ImageBuffer<RgbRaw<u8>, Vec<u8>>, _colour: vek::Rgb<u8>) {
        let font = deja_vu_sans();
        let mut rng = rand::thread_rng();

        for (i, e) in self.edges.iter().enumerate() {
            let col = crate::render::random_colour(&mut rng);
            e.line.draw(image, col);
            let center = (e.line.start + e.line.end) / (T::one() + T::one());
            draw_text(image, center.x, center.y, &format!("E{}", i), col, &font);
            vek::LineSegment2 {
                start: center,
                end: self.rects[e.a].rect.center(),
            }
            .draw(image, col / 4);
            vek::LineSegment2 {
                start: center,
                end: self.rects[e.b].rect.center(),
            }
            .draw(image, col / 4);
        }

        for (i, r) in self.rects.iter().enumerate() {
            let text = format!("R{}", i);
            let center = r.rect.center();
            let col = if r.active() {
                vek::Rgb::<u8>::green()
            } else {
                vek::Rgb::<u8>::red()
            };
            draw_text(image, center.x, center.y, &text, col, &font);
        }
    }
}

#[test]
fn treemap_test() {
    use crate::geom::Axis;
    #[cfg(feature = "rendering")]
    use crate::render::{Draw, ImageBuffer};
    use vek::Rect;

    let mut builder = super::TreemapBuilder::<f32>::new(Rect::new(0., 0., 510., 510.));
    builder.intersect_point(0, Axis::Horizontal, 0.25);
    builder.split(1, Axis::Vertical, 2);
    builder.intersect_point(2, Axis::Vertical, 0.75);
    builder.split(6, Axis::Horizontal, 2);
    builder.intersect_point(8, Axis::Vertical, 0.75);
    let map: Treemap<f32> = builder.build();

    #[cfg(feature = "rendering")]
    {
        let mut img = ImageBuffer::new(512, 512);
        map.draw(&mut img, vek::Rgb::red());
        img.save("tree_test.png").unwrap();
    }

    assert_eq!(map.rect_count(), 8);
    assert_eq!(map.edge_count(), 25);
}
