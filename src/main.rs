mod grid;
mod matrix;
mod vector;

pub use crate::vector::vector2::Vector2;
pub use crate::vector::vector4::Vector4;
use crate::{
    matrix::{
        matrix4::Matrix4,
        rotation::{Angle, Rotation},
        scale::Scale,
    },
    vector::{vector::Vector, vector3::Vector3},
};
pub use grid::Grid;

const WIDTH: usize = 100;
const HEIGHT: usize = 50;

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
    let mut vec = (vec + 1.0) / 2.0;
    vec.x *= WIDTH as f32;
    vec.y *= HEIGHT as f32;
    vec
}

fn main() {
    let mut grid = Grid::new(' ', WIDTH, HEIGHT);

    let tri: Triangle = Triangle {
        a: Vector3::new(-0.215, -0.5, 0.0),
        b: Vector3::new(0.8, -0.4, 0.0),
        c: Vector3::new(0.5, 0.3, 0.0),
    };

    let mut pitch = 0.0;
    let mut yaw = 0.0;
    let mut roll = 0.0;

    loop {
        let Triangle { a, b, c } = tri;

        // Convert to homogenous coordinates
        let a = a.homogenous();
        let b = b.homogenous();
        let c = c.homogenous();

        // Scaling matrix
        let scale = Matrix4::scale(0.75);

        // Rotation matrix
        let rotation = Matrix4::rotation(
            Angle::Degrees(pitch),
            Angle::Degrees(yaw),
            Angle::Degrees(roll),
        );

        // Translation matrix
        let position = Vector3::new(0.5, 0.0, 0.75);
        let translation = Matrix4::translation(position);

        // Perspective matrix
        let fov = Angle::Degrees(90.0);
        let z_far = 1000.0;
        let z_near = 0.1;
        let aspect = (WIDTH as f32) / (HEIGHT as f32);
        let perspective = Matrix4::perspective(fov, z_far, z_near, aspect);

        // Transform points using matrices
        let a = perspective * translation * rotation * scale * a;
        let b = perspective * translation * rotation * scale * b;
        let c = perspective * translation * rotation * scale * c;

        // Convert back to cartesian coordinates
        let a = a.cartesian();
        let b = b.cartesian();
        let c = c.cartesian();

        // Convert points to screen coordinates
        let a = to_screen_coordinates(a);
        let b = to_screen_coordinates(b);
        let c = to_screen_coordinates(c);

        let t = Triangle {a, b, c};

        // Calculate triangle bounding box
        let min_x = f32::min(a.x, f32::min(b.x, c.x));
        let min_y = f32::min(a.y, f32::min(b.y, c.y));
        let max_x = f32::max(a.x, f32::max(b.x, c.x));
        let max_y = f32::max(a.y, f32::max(b.y, c.y));

        let min_x = usize::clamp(min_x as usize, 0, WIDTH);
        let min_y = usize::clamp(min_y as usize, 0, HEIGHT);
        let max_x = usize::clamp(max_x as usize, 0, WIDTH);
        let max_y = usize::clamp(max_y as usize, 0, HEIGHT);

        for y in min_y..max_y {
            for x in min_x..max_x {
                let p = Vector3::new(x, y, 0.0);

                // Check whether pixel is close to
                if check_inside(&t, &p) {
                    grid.set('#', x as usize, y as usize);
                }
            }
        }
        print!("{grid}");
        print!("\x1B[2J\x1B[1;1H");
        grid.clear(' ');

        // roll += 0.5;
        yaw += 0.1;
        pitch += 0.1;
    }
}
