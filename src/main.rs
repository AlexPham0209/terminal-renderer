mod grid;
mod matrix;
mod vector;

use core::f32;
use std::time::Duration;

pub use crate::vector::vector2::Vector2;
pub use crate::vector::vector4::Vector4;
use crate::{
    matrix::{
        matrix::Matrix, matrix3::Matrix3, matrix4::Matrix4, rotation::{Angle, Rotation}, scale::Scale
    },
    vector::{vector::Vector, vector3::Vector3},
};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, poll, read};
pub use grid::Grid;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

struct Triangle {
    a: Vector3,
    b: Vector3,
    c: Vector3,
}

impl Triangle {
    fn get_bounding_box(&self) -> (usize, usize, usize, usize) {
        // Calculate triangle's bounding box
        let min_x = f32::min(self.a.x, f32::min(self.b.x, self.c.x));
        let min_y = f32::min(self.a.y, f32::min(self.b.y, self.c.y));
        let max_x = f32::max(self.a.x, f32::max(self.b.x, self.c.x));
        let max_y = f32::max(self.a.y, f32::max(self.b.y, self.c.y));

        let min_x = usize::clamp(min_x as usize, 0, WIDTH);
        let min_y = usize::clamp(min_y as usize, 0, HEIGHT);
        let max_x = usize::clamp(max_x as usize, 0, WIDTH);
        let max_y = usize::clamp(max_y as usize, 0, HEIGHT);

        (min_x, min_y, max_x, max_y)
    }
}

#[derive(Debug)]
struct Transform {
    yaw: f32,
    pitch: f32,
    roll: f32,

    dx: f32,
    dy: f32,
    dz: f32,

    position: Vector3,
    scale: f32,
    value: char
}

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

// fn check_inside(tri: &Triangle, p: &Vector3) -> bool {
//     let Triangle { a, b, c } = tri;
//     let abp = edge_function(a, b, p) >= 0.;
//     let bcp = edge_function(b, c, p) >= 0.;
//     let cap = edge_function(c, a, p) >= 0.;

//     abp == bcp && bcp == cap
// }

fn to_screen_coordinates(vec: Vector3) -> Vector3 {
    let Vector3 { x, y, z } = vec;

    Vector3::new(
        ((x + 1.0) / 2.0) * (WIDTH as f32),
        ((y + 1.0) / 2.0) * (HEIGHT as f32),
        z,
    )
}

fn rasterize_triangle(t: Triangle, grid: &mut Grid<char>, depth_buffer: &mut Grid<f32>, value: char)  {
    let Triangle {a, b, c} = t;

    // Skip if any of the points are behind the camera
    if a.z < 0.0 || b.z < 0.0 || c.z < 0.0 || a.z > 1.0 || b.z > 1.0 || c.z > 1.0 {
        return;
    }

    let (min_x, min_y, max_x, max_y) = t.get_bounding_box();
    let abc = edge_function(a, b, c);

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

            let depths = 1.0 / Vector3::new(
                a.z, b.z, c.z
            );

            // Barycentric coordinates
            let weights = Vector3::new(
                abp / abc, bcp / abc, cap / abc
            );
            
            // Calculates the depth and uses it to determine whether current pixel is has lowest depth
            let depth = 1.0 / depths.dot(weights);
            if let Some(prev) = depth_buffer.get(x, y) && depth >= *prev {
                continue;
            }

            depth_buffer.set(depth, x, y);
            grid.set(value, x as usize, y as usize);
        }
    }
}   

fn main() {
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
        yaw: 0.0,
        pitch: 0.0,
        roll: 0.0,

        dx: 1.0,
        dy: 1.0,
        dz: 0.0,

        position: Vector3::new(0.2, 0.0, 1.0),
        scale: 1.0,
        value: '.'
    };

    let mut transform2 = Transform {
        yaw: 30.0,
        pitch: 100.0,
        roll: 50.0,

        dx: 2.0,
        dy: 0.0,
        dz: 3.0,

        position: Vector3::new(0.6, -0.5, 2.0),
        scale: 0.5,
        value: '#',
    };

    let mut transform3 = Transform {
        yaw: 55.0,
        pitch: 180.0,
        roll: 12.0,

        dx: 0.0,
        dy: 1.0,
        dz: 3.0,

        position: Vector3::new(0, 1.0, 0),
        scale: 0.5,
        value: '/'
    };

    let mut triangles = vec![(tri1, transform1), (tri2, transform2), (tri3, transform3)];

    let mut camera_position = Vector3::new(0, 0, 0);
    let mut camera_pitch = 0.0;
    let mut camera_yaw = 0.0;
    let mut camera_roll = 0.0;
    
    // In world coordinates
    let light = Vector3::new(0.0, 2.0, 0.0);

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
                dx,
                dy,
                dz,
                position,
                scale,
                value
            } = *transform;

            // Scaling matrix
            let scalar = Matrix4::scale(scale);

            // Rotation matrix
            let rotation = Matrix4::rotation(
                Angle::Degrees(yaw),
                Angle::Degrees(pitch),
                Angle::Degrees(roll),
            );

            // Translation matrix
            let translation = Matrix4::translation(position);

            // Calculating normal vector (in object space)
            let ab = b - a;
            let ac = c - a;
            let normal = ab.cross(ac).normalize();

            //Calculating normal matrix
            let model_inverse = Matrix4::scale(1.0/scale) * rotation.transpose() * Matrix4::translation(-position);
            let normal_matrix = model_inverse.transpose();
                
            // Converting normal to world space
            let normal = (normal_matrix * Vector4::to_vector4(normal, 0.0)).xyz().normalize();
            
            // Calculate light value
            let l = (light - normal).normalize();
            let value = f32::max(0.0, normal.dot(l));
            println!("{value}");
            let value = f32::round(value * 3.0) as usize;

            let character: char = "./$#".as_bytes()[value] as char;

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
            rasterize_triangle(t, &mut grid, &mut depth_buffer, character);

            transform.yaw += transform.dx;
            transform.pitch += transform.dy;
            transform.roll += transform.dz;
        }

        print!("{grid}");
        print!("\x1B[2J\x1B[1;1H");
        grid.clear(' ');
        depth_buffer.clear(f32::INFINITY);
    }

}
