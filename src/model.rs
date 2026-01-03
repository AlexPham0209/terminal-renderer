use std::{collections::HashMap, fs};

use crate::{Vector2, transform::Transform, triangle::Triangle, vector::vector3::Vector3};

#[derive(Debug)]
pub struct Vertex {
    pos: Vector3,
    tex_coord: Option<Vector2>,
    normal: Option<Vector3>,
}

pub struct Model {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
    pub transform: Transform,
}

impl Model {
    pub fn new(path: &str) -> Option<i32> {
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

        for face in faces {
            for vertex in face {
                let vertex = vertex
                    .split("/")
                    .into_iter()
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<usize>>();

                let index = *vertex.get(0).unwrap() - 1;

                let pos = *vertices.get(index).expect("Index out of range");

                let tex_coord: Option<Vector2> = match vertex.get(1) {
                    Some(index) => Some(*tex_coords.get(*index - 1).expect("Index out of range")),
                    None => None,
                };

                let normal: Option<Vector3> = match vertex.get(2) {
                    Some(index) => Some(*normals.get(*index - 1).expect("Index out of range")),
                    None => None,
                };

                let vertex = Vertex {
                    pos,
                    tex_coord,
                    normal,
                };

                println!("{:?}", vertex);
            }
        }
        // println!("{:?}", normals);
        Some(0)
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
}
