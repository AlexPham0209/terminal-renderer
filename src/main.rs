mod grid;
mod matrix;
mod model;
mod transform;
mod triangle;
mod vector;
mod vertex;

use core::f32;
use std::time::Duration;

pub use crate::vector::vector2::Vector2;
pub use crate::vector::vector4::Vector4;
use crate::{
    matrix::{
        matrix::Matrix,
        matrix3::Matrix3,
        matrix4::Matrix4,
        rotation::{Angle, Rotation},
        scale::Scale,
    },
    model::{Model, VertexData},
    transform::Transform,
    triangle::Triangle,
    vector::{vector::Vector, vector3::Vector3}, vertex::Vertex,
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, poll, read};
pub use grid::Grid;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

// Make sure that points are in counter-clockwise order
fn edge_function(a: Vector3, b: Vector3, c: Vector3) -> f32 {
    // Calculates vector representing the line from point A to C
    let ac = c - a;

    // Calculate the vector representing the triangle edge (A to B)
    let ab = a - b;

    // Calculating the normal/perpendicular vector of the AB side.
    let ab_perp = Vector3::new(ab.y, -ab.x, ab.z);

    // The dot product calculates how similar the directions of two vectors are
    // If it is negative, then they are facing opposite directions
    // If it is positive, then they are facing similar directions
    // If the dot product between the normal and the AC vector are positive, then the vector is on the right side of the triangle
    ac.dot(ab_perp)
}

fn to_screen_coordinates(vec: Vector3) -> Vector3 {
    let Vector3 { x, y, z } = vec;

    Vector3::new(
        ((x + 1.0) / 2.0) * (WIDTH as f32),
        ((y + 1.0) / 2.0) * (HEIGHT as f32),
        z,
    )
}

fn get_normal(a: Vector3, b: Vector3, c: Vector3) -> Vector3 {
    let ab = b - a;
    let ac = c - a;
    ab.cross(ac).normalize()
}

fn rasterize_triangle(
    t: Triangle,
    grid: &mut Grid<char>,
    depth_buffer: &mut Grid<f32>,
    normal: Vector3,
    light: Vector3,
) {
    let Triangle { a, b, c } = t;

    // Skip if any of the points are behind the camera
    if a.z < 0.0 || b.z < 0.0 || c.z < 0.0 || a.z > 1.0 || b.z > 1.0 || c.z > 1.0 {
        return;
    }

    let (min_x, min_y, max_x, max_y) = t.get_bounding_box();
    let abc = edge_function(a, b, c);
    let gradient = "`.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";
    // Iterating through every pixel/point inside of triangle's bounding box
    for y in min_y..max_y {
        for x in min_x..max_x {
            let p = Vector3::new(x, y, 0.0);

            let abp = edge_function(a, b, p);
            let bcp = edge_function(b, c, p);
            let cap = edge_function(c, a, p);
            let is_inside = (abp >= 0.0) == (bcp >= 0.0) && (bcp >= 0.0) == (cap >= 0.0);

            if !is_inside {
                continue;
            }

            // Barycentric coordinates
            let weights = Vector3::new(abp / abc, bcp / abc, cap / abc);

            let depths = 1.0 / Vector3::new(a.z, b.z, c.z);
            let depth = 1.0 / depths.dot(weights);

            // // Interpolated normals
            // let n = ((normal_1 * depths.x) * weights.x)
            //     + ((normal_2 * depths.y) * weights.y)
            //     + ((normal_3 * depths.z) * weights.z);
            // let n = n * depth;
            // let n = n.normalize();

            // Calculating light value
            let l = (light - normal).normalize();
            let value = (normal.dot(l) + 1.0) / 2.0;
            let value = f32::round(value * ((gradient.len() - 1) as f32)) as usize;
            let value: char = gradient.as_bytes()[value] as char;

            // Calculates the depth and uses it to determine whether current pixel is has lowest depth
            if let Some(prev) = depth_buffer.get(x, y)
                && depth >= *prev
            {
                continue;
            }

            depth_buffer.set(depth, x, y);
            grid.set(value, x as usize, y as usize);
        }
    }
}

fn demo() {
    let mut grid = Grid::new(' ', WIDTH, HEIGHT);
    let mut depth_buffer: Grid<f32> = Grid::new(f32::INFINITY, WIDTH, HEIGHT);

    let tri1: Triangle = Triangle {
        a: Vector3::new(0.5, -0.5, 0.0),
        b: Vector3::new(-0.5, -0.5, 0.0),
        c: Vector3::new(0.0, 0.5, 0.0),
    };

    let tri2: Triangle = Triangle {
        a: Vector3::new(0.15, -0.25, 0.0),
        b: Vector3::new(-0.5, -0.45, 0.0),
        c: Vector3::new(0.0, 0.5, 0.0),
    };

    let tri3: Triangle = Triangle {
        a: Vector3::new(0.15, -0.25, 0.0),
        b: Vector3::new(-0.5, -0.45, 0.0),
        c: Vector3::new(0.0, 0.5, 0.0),
    };

    let mut transform1 = Transform {
        yaw: Angle::Degrees(30.0),
        pitch: Angle::Degrees(100.0),
        roll: Angle::Degrees(50.0),
        position: Vector3::new(0.2, 0.0, 1.0),
        scale: 0.25,
    };

    let mut transform2 = Transform {
        yaw: Angle::Degrees(30.0),
        pitch: Angle::Degrees(100.0),
        roll: Angle::Degrees(50.0),
        position: Vector3::new(0.6, -0.5, 2.0),
        scale: 0.5,
    };

    let mut transform3 = Transform {
        yaw: Angle::Degrees(30.0),
        pitch: Angle::Degrees(100.0),
        roll: Angle::Degrees(50.0),
        position: Vector3::new(0, 0.0, 0.0),
        scale: 0.75,
    };

    let mut triangles = vec![(tri1, transform1), (tri2, transform2), (tri3, transform3)];

    let mut camera_position = Vector3::new(0, 0, 0);
    let mut camera_pitch = 0.0;
    let mut camera_yaw = 0.0;
    let mut camera_roll = 0.0;

    // In world coordinates
    let light = Vector3::new(0.5, -3.0, 0.5);

    // Perspective matrix
    let fov = Angle::Degrees(90.0);
    let z_far = 10000.0;
    let z_near = 0.1;
    let aspect = (WIDTH as f32) / (HEIGHT as f32);
    let perspective = Matrix4::perspective(fov, z_far, z_near, aspect);

    loop {
        // Use column vectors of rotation matrix for forward and right vectors
        let direction: Matrix3 = Matrix3::rotation(
            Angle::Degrees(camera_yaw),
            Angle::Degrees(camera_pitch),
            Angle::Degrees(camera_roll),
        );

        let forward = direction.z;
        let right = direction.x;

        if poll(Duration::from_millis(10)).unwrap() {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('w'),
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                }) => camera_position = camera_position - forward * 0.1,

                Event::Key(KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                }) => camera_position = camera_position + forward * 0.1,

                Event::Key(KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                }) => camera_position = camera_position - right * 0.1,

                Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                }) => camera_position = camera_position + right * 0.1,

                // Camera controls
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                }) => camera_yaw -= 2.0,

                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                }) => camera_yaw += 2.0,

                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                }) => camera_pitch += 2.0,

                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE,
                    kind: _,
                    state: _,
                }) => camera_pitch -= 2.0,

                _ => {}
            }
        }

        // View matrix
        let view = Matrix4::view(
            Angle::Degrees(camera_yaw),
            Angle::Degrees(camera_pitch),
            Angle::Degrees(camera_roll),
            camera_position,
        );

        for (tri, transform) in &mut triangles {
            let Triangle { a, b, c } = *tri;
            let Transform {
                yaw,
                roll,
                pitch,

                position,
                scale,
            } = *transform;

            // Scaling matrix
            let scalar = Matrix4::scale(scale);

            // Rotation matrix
            let rotation = Matrix4::rotation(yaw, pitch, roll);

            // Translation matrix
            let translation = Matrix4::translation(position);

            // Calculating normal vectors for each vertex (in object space)
            let normal = get_normal(a, b, c);

            //Calculating world normal matrix
            let model_inverse = Matrix3::scale(1.0 / scale) * rotation.cartesian().transpose();
            let normal_matrix = model_inverse.transpose();

            // Converting normal vectors to world space
            let normal = (normal_matrix * normal).normalize();

            // Transform points using matrices
            let a = perspective * view * translation * rotation * scalar * a;
            let b = perspective * view * translation * rotation * scalar * b;
            let c = perspective * view * translation * rotation * scalar * c;

            // Convert points to screen coordinates
            let a = to_screen_coordinates(a);
            let b = to_screen_coordinates(b);
            let c = to_screen_coordinates(c);

            // Final projected triangle
            let t = Triangle { a, b, c };
            rasterize_triangle(t, &mut grid, &mut depth_buffer, normal, light);
        }

        print!("{grid}");
        print!("\x1B[2J\x1B[1;1H");
        grid.clear(' ');
        depth_buffer.clear(f32::INFINITY);
    }
}


fn main() {
    let model = Model::load("bin/teapot.obj").unwrap();

    for (a, b, c) in &model.data {
        
        let a = Vertex::new(a, &model);
        let b = Vertex::new(b, &model);
        let c = Vertex::new(c, &model);
        
        println!("{:?}", a);
        println!("{:?}", b);
        println!("{:?}\n", c);
    }
}
