mod grid;
mod matrix;
mod vector;

use std::time::Duration;

pub use crate::vector::vector2::Vector2;
pub use crate::vector::vector4::Vector4;
use crate::{
    matrix::{
        matrix3::Matrix3, matrix4::Matrix4, rotation::{Angle, Rotation}, scale::Scale
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

// Make sure that points are in counter-clockwise order
fn edge_function(a: &Vector3, b: &Vector3, c: &Vector3) -> f32 {
    // Calculates vector representing the line from point A to C
    let ac = *c - *a;

    // Calculate the vector representing the triangle edge (A to B)
    let ab = *a - *b;

    // Calculating the normal/perpendicular vector of the AB side.
    let ab_perp = Vector3::new(ab.y, -ab.x, ab.z);

    // The dot product calculates how similar the directions of two vectors are
    // If it is negative, then they are facing opposite directions
    // If it is positive, then they are facing similar directions
    // If the dot product between the normal and the AC vector are positive, then the vector is on the right side of the triangle
    ac.dot(ab_perp)
}

fn check_inside(tri: &Triangle, p: &Vector3) -> bool {
    let Triangle { a, b, c } = tri;
    let abp = edge_function(a, b, p) >= 0.;
    let bcp = edge_function(b, c, p) >= 0.;
    let cap = edge_function(c, a, p) >= 0.;

    abp == bcp && bcp == cap
}

fn to_screen_coordinates(vec: Vector3) -> Vector3 {
    let Vector3 { x, y, z} = vec;

    Vector3::new(
        ((x + 1.0) / 2.0) * (WIDTH as f32), 
        ((y + 1.0) / 2.0) * (HEIGHT as f32), 
        z
    )
}

fn main() {
    let mut grid = Grid::new(' ', WIDTH, HEIGHT);

    let tri: Triangle = Triangle {
        a: Vector3::new(0.5, -0.5, 0.0),
        b: Vector3::new(-0.5, -0.5, 0.0),
        c: Vector3::new(0.0, 0.5, 0.0),
    };

    let mut pitch = 0.0;
    let mut yaw = 0.0;
    let mut roll = 0.0;

    let mut camera_position = Vector3::new(0, 0, 0);
    let mut camera_pitch = 0.0;
    let mut camera_yaw = 0.0;
    let mut camera_roll = 0.0;


    loop {

        // Use column vectors of rotation vector to create forward and right vectors
        let direction: Matrix3 = Matrix3::rotation(
        Angle::Degrees(camera_yaw),
        Angle::Degrees(camera_pitch),
        Angle::Degrees(camera_roll), 
        );

        let forward = direction.z;
        let right = direction.x;

        if poll(Duration::from_millis(30)).unwrap() {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('w'),
                    modifiers: KeyModifiers::NONE, 
                    kind: _, 
                    state: _ 
                }) => camera_position = camera_position + forward * 0.1,

                Event::Key(KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::NONE, 
                    kind: _, 
                    state: _ 
                }) => camera_position = camera_position - forward * 0.1,

                Event::Key(KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::NONE, 
                    kind: _, 
                    state: _ 
                }) => camera_position = camera_position + right * 0.1,

                Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::NONE, 
                    kind: _, 
                    state: _ 
                }) => camera_position = camera_position - right * 0.1,

                // Camera controls
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    modifiers: KeyModifiers::NONE, 
                    kind: _, 
                    state: _ 
                }) => camera_yaw -= 2.0,

                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    modifiers: KeyModifiers::NONE, 
                    kind: _, 
                    state: _ 
                }) => camera_yaw += 2.0,

                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::NONE, 
                    kind: _, 
                    state: _ 
                }) => camera_pitch += 2.0,

                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::NONE, 
                    kind: _, 
                    state: _ 
                }) => camera_pitch -= 2.0,
                
                _ => {}
            }
        }
        
        let Triangle { a, b, c } = tri;

        // Convert to homogenous coordinates
        let a = a.homogenous();
        let b = b.homogenous();
        let c = c.homogenous();

        // Scaling matrix
        let scale = Matrix4::scale(1.0);

        // Rotation matrix
        let rotation = Matrix4::rotation(
            Angle::Degrees(pitch),
            Angle::Degrees(yaw),
            Angle::Degrees(roll),
        );
        
        // Translation matrix
        let position = Vector3::new(0.0, 0.0, 1.0);
        let translation = Matrix4::translation(position);
        
        // View matrix
        let view = Matrix4::view(
            Angle::Degrees(camera_yaw),
            Angle::Degrees(camera_pitch),
            Angle::Degrees(camera_roll), 
            camera_position
        );

        // Perspective matrix
        let fov = Angle::Degrees(90.0);
        let z_far = 10000.0;
        let z_near = 0.1;
        let aspect = (WIDTH as f32) / (HEIGHT as f32);
        let perspective = Matrix4::perspective(fov, z_far, z_near, aspect);

        // Transform points using matrices
        let a = perspective * view * translation * rotation * scale * a;
        let b = perspective * view * translation * rotation * scale * b;
        let c = perspective * view * translation * rotation * scale * c;

        // Convert back to cartesian coordinates
        let a = a.cartesian();
        let b = b.cartesian();
        let c = c.cartesian();

        // Convert points to screen coordinates
        let a = to_screen_coordinates(a);
        let b = to_screen_coordinates(b);
        let c = to_screen_coordinates(c);
    

        // Final projected triangle
        let t = Triangle {a, b, c};

        // Calculate triangle's bounding box
        let min_x = f32::min(a.x, f32::min(b.x, c.x));
        let min_y = f32::min(a.y, f32::min(b.y, c.y));
        let max_x = f32::max(a.x, f32::max(b.x, c.x));
        let max_y = f32::max(a.y, f32::max(b.y, c.y));

        let min_x = usize::clamp(min_x as usize, 0, WIDTH);
        let min_y = usize::clamp(min_y as usize, 0, HEIGHT);
        let max_x = usize::clamp(max_x as usize, 0, WIDTH);
        let max_y = usize::clamp(max_y as usize, 0, HEIGHT);

        // Iterating through every pixel/point inside of triangle's bounding box
        for y in min_y..max_y {
            for x in min_x..max_x {
                let p = Vector3::new(x, y, 0.0);

                // Skip if any of the points are behind the camera
                if a.z < 0.0 || b.z < 0.0 || c.z < 0.0 || a.z > 1.0 || b.z > 1.0 || c.z > 1.0 {
                    continue
                }

                // Check whether pixel is inside of triangle
                if check_inside(&t, &p) {
                    grid.set('#', x as usize, y as usize);
                }
            }
        }
        print!("{grid}");
        print!("\x1B[2J\x1B[1;1H");
        grid.clear(' ');

        roll += 1.0;
        yaw += 1.0;
        pitch += 1.0;
    }
}
