use super::{Mesh};
use crate::core::GeoNum;
use crate::geom::Vertex;
use std::fmt::{Debug};

pub trait Filter<T>: Debug where T: GeoNum {
    fn apply(&self, mesh: &mut Mesh<T>);
    //fn render(&self, mesh: &Mesh<T>) -> Result<Mesh<T>, Error>;
}

#[derive(Debug)]
pub struct Scale<T> where T: GeoNum {
    amount: T
}

impl<T> Scale<T> where T: GeoNum {
    pub fn new(amount: T) -> Self {
        Self {
            amount
        }
    }

    fn map_vert(&self, vert: &Vertex<T>) -> Vertex<T> {
        *vert * self.amount
    }
}

impl<T> Filter<T> for Scale<T> where T: GeoNum {
    fn apply(&self, mesh: &mut Mesh<T>) {
        mesh.map_verts(|v| self.map_vert(v));
    }
}


#[test]
fn scale_test() {
    let mut mesh = Mesh::<f64>::new();
    let filter = Scale::new(2.);
    mesh.add_filter(Box::new(filter));
    let _result = mesh.render_filters().unwrap();
}