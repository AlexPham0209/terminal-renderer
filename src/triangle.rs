use crate::{HEIGHT, WIDTH, vector::vector3::Vector3, vertex::Vertex};

#[derive(Debug)]
pub struct Triangle<'a> {
    pub a: Vertex<'a>,
    pub b: Vertex<'a>,
    pub c: Vertex<'a>,
}

impl<'a> Triangle<'a> {
    pub fn get_bounding_box(&self) -> (usize, usize, usize, usize) {
        let a = self.a.pos;
        let b = self.b.pos;
        let c = self.c.pos;

        // Calculate triangle's bounding box
        let min_x = f32::min(a.x, f32::min(b.x, c.x));
        let min_y = f32::min(a.y, f32::min(b.y, c.y));
        let max_x = f32::max(a.x, f32::max(b.x, c.x));
        let max_y = f32::max(a.y, f32::max(b.y, c.y));

        let min_x = usize::clamp(min_x as usize, 0, WIDTH);
        let min_y = usize::clamp(min_y as usize, 0, HEIGHT);
        let max_x = usize::clamp(max_x as usize, 0, WIDTH);
        let max_y = usize::clamp(max_y as usize, 0, HEIGHT);

        (min_x, min_y, max_x, max_y)
    }
}
