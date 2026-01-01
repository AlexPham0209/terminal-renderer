mod grid;
mod matrix;
mod vector;

pub use crate::vector::vector2::Vector2;
use crate::{matrix::{matrix4::Matrix4, rotation::{Angle, Rotation}, scale::Scale}, vector::vector3::Vector3};
pub use crate::vector::vector4::Vector4;
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
    ((c.y - a.y) * (b.x - a.x)) - ((c.x - a.x) * (b.y - a.y))
}

fn check_inside(tri: &Triangle, p: &Vector3) -> bool {
    let Triangle { a, b, c } = tri;
    let abp = edge_function(a, b, p) >= 0.;
    let bcp = edge_function(b, c, p) >= 0.;
    let cap = edge_function(c, a, p) >= 0.;

    abp && bcp && cap || (!abp && !bcp && !cap)
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
        a: Vector3::new(-0.5, -0.5, 0.0),
        b: Vector3::new(0.5, -0.5, 0.0),
        c: Vector3::new(0.0, 0.5, 0.0),
    };

    let mut pitch = 0.0;
    let mut yaw = 0.0;
    let mut roll = 0.0;

    loop {
        for y in 0..grid.height {
            for x in 0..grid.width {
                let Triangle {a, b, c} = tri;
                let p = Vector3::new(x, y, 0.0);

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
                let position = Vector3::new(0.0, 0.0, 0.5);
                let translation = Matrix4::translation(position);

                // Perspective matrix
                let fov = Angle::Degrees(90.0);
                let z_far = 1000.0;
                let z_near= 0.1;
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
        yaw += 5.0;
        pitch += 5.0;

    }
}

