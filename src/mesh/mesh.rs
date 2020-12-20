use vek::Vec3;
use super::{Face, FaceIndex, Primitive, Filter};
use crate::core::{PointIndex, GeoNum};
use crate::geom::Vertex;
use std::fmt::{Error, Debug};

pub trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
pub struct Mesh<T> where T: GeoNum {
    verticies: Vec<Vertex<T>>,
    faces: Vec<Face>,
    name: Option<String>,
    filters: Vec<Box<dyn Filter<T>>>,
}

impl<T> Clone for Mesh<T> where T: GeoNum {
    /// cloning a mesh will loose all filters. 
    fn clone(&self) -> Self {
        Self {
            verticies: self.verticies.clone(),
            faces: self.faces.clone(),
            name: self.name.clone(),
            filters: Vec::new(),
        }
    }

}

impl<T> Mesh<T> where T: GeoNum {

    pub fn new() -> Self {
        Self {
            verticies: Vec::new(),
            faces: Vec::new(),
            name: None,
            filters: Vec::new(),
        }
    }

    /// sets the name as it will appear in any formatted files, such as .obj or .fbx
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    /// adds a lone vertex to the mesh. Be sure to give him some friends!
    pub fn add_vertex(&mut self, vertex: Vertex<T>) -> PointIndex {
        let i = self.verticies.len();
        self.verticies.push(vertex);
        PointIndex::new(i)
    }

    /// adds a face to the mesh. Assumes the vertecies are already in the mesh
    pub fn add_face(&mut self, face: Face) -> FaceIndex {
        let i = self.faces.len();
        self.faces.push(face);
        FaceIndex::new(i)
    }

    /// generates a face from given points and adds the vertecies to the mesh. Not to be used in conjunction with add_vertex or add_face
    pub fn make_face(&mut self, verticies: Vec<Vertex<T>>) -> FaceIndex {
        let mut face = Face::capacity(verticies.len());
        for i in 0..verticies.len() {
            let vi = self.add_vertex(verticies[i]);
            face.add_vert(vi);
        }
        self.add_face(face)
    }

    /// adds a filter to our lovely mesh
    pub fn add_filter(&mut self, filter: Box<dyn Filter<T>>) {
        self.filters.push(filter);
    }

    /// translates the mesh using given Vec3
    pub fn translate(&mut self, offset: Vec3<T>) {
        self.map_verts(|v| Vertex::new(v.x + offset.x, v.y + offset.y, v.z + offset.z));
    }

    /// inverts the sign of all x coordinates
    pub fn invert_x(&mut self) {
        self.map_verts(|v| 
            Vertex::new(v.x * -T::one(), v.y, v.z)
        );
    }

    /// inverts the sign of all y coordinates
    pub fn invert_y(&mut self) {
        self.map_verts(|v| 
            Vertex::new(v.x, v.y * -T::one(), v.z)
        );
    }

    /// inverts the sign of all z coordinates
    pub fn invert_z(&mut self) {
        self.map_verts(|v| 
            Vertex::new(v.x, v.y, v.z * -T::one())
        );
    }

    /// returns the given name of this mesh
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    /// applies a function to the verticies of this mesh
    pub fn map_verts<F>(&mut self, f: F)  
        where F: Fn(&Vertex<T>) -> Vertex<T> 
    {
        self.verticies = self.verticies.iter().map(|x| f(&x)).collect();
    }

    /// makes a copy of the mesh and applies all filters
    pub fn render_filters(&self) -> Result<Mesh<T>, Error> {
        let mut mesh = self.clone();

        for filter_box in self.filters.iter() {
            filter_box.as_ref().apply(&mut mesh);
        }
        Ok(mesh)
    }
}

impl<T> Primitive<T> for Mesh<T> where T: GeoNum {
    /// Getter for verts
    fn verticies(&self) -> &Vec<Vertex<T>> {
        &self.verticies
    }

    /// getter for faces. Duh
    fn faces(&self) -> &Vec<Face> {
        &self.faces
    }

    fn vertex(&self, index: PointIndex) -> Option<&Vertex<T>> {
        if index.index() >= self.verticies.len() {
            return None;
        }
        Some(&self.verticies[index.index()])
    }

    fn vertex_mut(&mut self, index: PointIndex) -> Option<&mut Vertex<T>> {
        if index.index() >= self.verticies.len() {
            return None;
        }
        Some(&mut self.verticies[index.index()])
    }
}