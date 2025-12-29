use std::ops::Index;

use crate::Vector2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2 {
    x: Vector2,
    y: Vector2
}

impl Matrix2 {
    pub fn from_cols(x: Vector2, y: Vector2) -> Self {
        Self { x, y }
    } 
}

impl Index<usize> for Matrix2 {
    type Output = Vector2;

    fn index(&self, index: usize) -> &Vector2 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of range")
        }       
    }
}

mod tests {
    use super::*;

    #[test]
    fn indexing_rust() {
        let x = Vector2::new(10, 5);
        let y = Vector2::new(5, 2);
        let mat = Matrix2::from_cols(x, y);
        assert_eq!(mat[0][0], 10.0);
        assert_eq!(mat[0][1], 5.0);
        assert_eq!(mat[1][0], 5.0);
        assert_eq!(mat[1][1], 2.0);
    }
}