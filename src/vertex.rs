use crate::{Vector2, model::{Model, VertexData}, vector::vector3::Vector3};

#[derive(Debug)]
pub struct Vertex<'a> {
    pub pos: &'a Vector3,
    pub tex_coord: Option<&'a Vector2>,
    pub normal: Option<&'a Vector3>,
}

impl<'a> Vertex<'a> {
    pub fn new(data: &'a VertexData, model: &'a Model) -> Vertex<'a> {
        let VertexData { pos, tex_coord, normal } = data;
        let pos = model.vertices.get(data.pos - 1).unwrap();
        
        let tex_coord = match tex_coord {
            Some(index) => Some(model.tex_coords.get(*index - 1).expect("Expected valid tex coord index")),
            None => None,
        };  
        
        let normal: Option<&Vector3> = match normal {
            Some(index) => Some(model.normals.get(*index - 1).expect("Expected valid tex coord index")),
            None => None,
        };

        Vertex {
            pos,
            tex_coord,
            normal
        }
    }
}