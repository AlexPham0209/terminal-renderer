use crate::{HEIGHT, WIDTH, vector::vector3::Vector3};

#[derive(Debug)]
pub struct Triangle {
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3,
}

impl Triangle {
    pub fn get_bounding_box(&self) -> (usize, usize, usize, usize) {
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
