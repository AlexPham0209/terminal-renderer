mod grid;
mod matrix;
mod vector;

pub use crate::vector::vector2::Vector2;
pub use crate::vector::vector4::Vector4;
pub use grid::Grid;

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

struct Triangle {
    a: Vector4,
    b: Vector4,
    c: Vector4,
}

// Make sure that points are in counter-clockwise order
fn edge_function(a: &Vector4, b: &Vector4, c: &Vector4) -> f32 {
    ((c.y - a.y) * (b.x - a.x)) - ((c.x - a.x) * (b.y - a.y))
}

fn check_inside(tri: &Triangle, p: &Vector4) -> bool {
    let Triangle { a, b, c } = tri;
    let abp = edge_function(a, b, p) >= 0.;
    let bcp = edge_function(b, c, p) >= 0.;
    let cap = edge_function(c, a, p) >= 0.;

    abp && bcp && cap
}

fn main() {
    let mut grid = Grid::new('.', WIDTH, HEIGHT);

    let tri: Triangle = Triangle {
        a: Vector4 {
            x: -1.0,
            y: -0.5,
            z: 0.0,
            w: 1.0,
        },
        b: Vector4 {
            x: 0.75,
            y: -0.75,
            z: 0.0,
            w: 1.0,
        },
        c: Vector4 {
            x: -0.5,
            y: 0.55,
            z: 0.0,
            w: 1.0,
        },
    };

    for y in 0..grid.height {
        for x in 0..grid.width {
            let x = x as f32;
            let y = y as f32;

            let w = WIDTH as f32 - 1.;
            let h = HEIGHT as f32 - 1.;

            // Convert (x, y) from range [0, 1] to [-1, 1]
            // Also invert the y coordinate to make the top of the coordinate space 1 and the bottom 1
            let p: Vector4 = Vector4 {
                x: ((x / w) * 2.) - 1.,
                y: -(((y / h) * 2.) - 1.),
                z: 0.,
                w: 1.,
            };

            // Check whether pixel is close to
            if check_inside(&tri, &p) {
                grid.set('#', x as usize, y as usize);
            }
        }
    }
    println!("{grid}");
}

