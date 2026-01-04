use std::{collections::HashMap, fs};

use crate::{Vector2, matrix::rotation::Angle, triangle::Triangle, vector::vector3::Vector3};


#[derive(Debug, Clone, Copy)]
pub struct VertexData {
    pub pos: usize,
    pub tex_coord: Option<usize>,
    pub normal: Option<usize>,
}

#[derive(Debug)]
pub struct Model {
    pub data: Vec<(VertexData, VertexData, VertexData)>,
    pub vertices: Vec<Vector3>,
    pub tex_coords: Vec<Vector2>,
    pub normals: Vec<Vector3>,
    pub transform: Transform
}

#[derive(Debug)]
pub struct Transform {
    pub yaw: Angle,
    pub pitch: Angle,
    pub roll: Angle,
    pub position: Vector3,
    pub scale: f32,
}


impl Model {
    pub fn load(path: &str) -> Option<Model> {
        // Reading obj file
        let data: Vec<String> = match fs::read_to_string(path) {
            Ok(data) => data.lines().map(String::from).collect(),
            Err(_) => return Option::None,
        };

        let mut vertices: Vec<Vector3> = Vec::new();
        let mut normals: Vec<Vector3> = Vec::new();
        let mut tex_coords: Vec<Vector2> = Vec::new();
        let mut faces: Vec<Vec<&str>> = Vec::new();

        for line in &data {
            let line = line.split(' ').collect::<Vec<&str>>();
            let (command, parameters) = line.split_first().expect("Incomplete line");

            match *command {
                "v" => {
                    if let Some(vertex) = Model::to_vector3(&parameters.to_vec()) {
                        vertices.push(vertex);
                    }
                }
                "vn" => {
                    if let Some(normal) = Model::to_vector3(&parameters.to_vec()) {
                        normals.push(normal);
                    }
                }

                "vt" => {
                    if let Some(tex_coord) = Model::to_vector2(&parameters.to_vec()) {
                        tex_coords.push(tex_coord);
                    }
                }
                "f" => faces.push(parameters.to_vec()),
                _ => continue,
            }
        }

        let mut data: Vec<(VertexData, VertexData, VertexData)> = Vec::new();
        for face in &faces {
            let mut f: Vec<VertexData> = Vec::new();
            
            for vertex in face {
                let vertex = vertex
                    .split("/")
                    .into_iter()
                    .map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<Option<usize>>>();
                
                let pos = (*vertex.get(0).unwrap()).unwrap();
                let tex_coord = *vertex.get(1).unwrap_or(&None);
                let normal = *vertex.get(2).unwrap_or(&None);
                    
                let vertex = VertexData {
                    pos,
                    tex_coord,
                    normal,
                };

                f.push(vertex);
            }

            if f.len() == 3 {
                data.push((f[0], f[1], f[2]));
            }
        }

        let transform = Transform {
            yaw: Angle::Degrees(0.0),
            pitch: Angle::Degrees(0.0),
            roll: Angle::Degrees(0.0),
            position: Vector3::new(0.0, 0.0, 0.0),
            scale: 0.1
        };

        let model = Model {
            data,
            vertices,
            normals,
            tex_coords,
            transform
        };

        Some(model)
    }

    fn to_vector3(vert: &Vec<&str>) -> Option<Vector3> {
        let vert: Vec<f32> = vert.iter().filter_map(|s| s.parse::<f32>().ok()).collect();
        if vert.len() != 3 {
            return None;
        }
        Some(Vector3::new(vert[0], vert[1], vert[2]))
    }

    fn to_vector2(vert: &Vec<&str>) -> Option<Vector2> {
        let vert: Vec<f32> = vert.iter().filter_map(|s| s.parse::<f32>().ok()).collect();
        if vert.len() != 2 {
            return None;
        }

        Some(Vector2::new(vert[0], vert[1]))
    }

    // fn triangles(&self) {
        
    // }
    
}
