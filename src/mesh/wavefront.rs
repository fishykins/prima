use crate::mesh::*;
use crate::core::{GeoNum, PointIndex};
use crate::geom::Vertex;
use std::io::{Error, BufRead};
use std::io::prelude::*;
use std::fs::File;
use obj::{raw::{parse_obj as parse_external, object::Polygon}};

pub fn parse<T, B>(input: B) -> Result<Mesh<T>, Error> 
    where 
        T: GeoNum, 
        B: BufRead 
{
    // Parse using external tool
    let raw = parse_external(input).unwrap();
    let mut mesh = Mesh::<T>::new();

    for p in raw.positions {
        let x = T::from_f32(p.0).unwrap();
        let y = T::from_f32(p.2).unwrap();
        let z = T::from_f32(p.1).unwrap();
        mesh.add_vertex(Vertex::new(x, y, z));
    }

    for p in raw.polygons {
        match p {
            Polygon::P(face) => {
                mesh_add_face(&mut mesh, face);
            },
            Polygon::PT(face) => {
                mesh_add_face(&mut mesh, face.iter().map(|x| x.0).collect());
            },
            Polygon::PN(face) => {
                mesh_add_face(&mut mesh, face.iter().map(|x| x.0).collect());
            },
            Polygon::PTN(face) => {
                mesh_add_face(&mut mesh, face.iter().map(|x| x.0).collect());
            }
        }
    }

    return Ok(mesh);
}

pub fn export<T>(mesh: &Mesh<T>, file_path: String) -> std::io::Result<()> 
    where T: GeoNum
{
    let mut file = File::create(format!("{}.obj", file_path))?;
    let mut data = Vec::new();
    data.push(format!("# Generated for use in Torus"));
    data.push(format!("mtllib {}.mtl", file_path));
    let name = if mesh.name().is_some() {
        mesh.name().unwrap()
    } else {
        file_path.into()
    };
    data.push(format!("o {}", name));
    for vert in mesh.verticies().iter() {
        data.push(format!("v {} {} {}", vert.x, vert.y, vert.z));
    }

    for face in mesh.faces().iter() {
        let mut list = Vec::new();
        for v in face.verticies() {
            // Offset the indexing as .obj files start at index 1, not 0
            list.push(format!("{}", v.index() + 1));
        };
        data.push(format!("f {}", list.join(" ")));
    }

    file.write(data.join("\n").as_bytes())?;
    Ok(())
}

fn mesh_add_face<T: GeoNum>(mesh: &mut Mesh<T>, verts: Vec<usize>) {
    mesh.add_face(Face::new(verts.iter().map(|x| PointIndex::new(*x)).collect()));
}