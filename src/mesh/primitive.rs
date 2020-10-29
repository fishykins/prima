use crate::core::{GeoNum, PointIndex};
use crate::geom::Vertex;
use crate::mesh::{Face};

pub trait Primitive<T> where T: GeoNum {
    /// Getter for verts
    fn verticies(&self) -> &Vec<Vertex<T>>;
    fn faces(&self) -> &Vec<Face>;
    fn vertex(&self, index: PointIndex) -> Option<&Vertex<T>>;
    fn vertex_mut(&mut self, index: PointIndex) -> Option<&mut Vertex<T>>;
}