use std::ops::Index;

use crate::{Vector2, vector::vector3::Vector3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2 {
    x: Vector2,
    y: Vector2,
}

impl Matrix2 {
    pub fn from_rows(x: Vector2, y: Vector2) -> Self {
        Self { 
            x: Vector2::new(x[0], y[0]), 
            y: Vector2::new(x[1], y[1])
        }
    } 
    pub fn from_cols(x: Vector2, y: Vector2) -> Self {
        Self { 
            x, 
            y,
        }
    } 

    pub fn row(&self, index: usize) -> Vector2 {
        match index {
            0 => Vector2::new(self.x[0], self.y[0]),
            1 => Vector2::new(self.x[1], self.y[1]),
            _ => panic!("Out of range")
        }
    }

    pub fn col(&self, index: usize) -> Vector2 {
        match index {
            0 => self.x,
            1 => self.y,
            _ => panic!("Out of range")
        }
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
    fn indexing_test() {
        let x = Vector2::new(10, 5);
        let y = Vector2::new(7, 2);
        let mat = Matrix2::from_cols(x, y);

        // First index (column), second index (Row)
        assert_eq!(mat[0][0], 10.0);
        assert_eq!(mat[0][1], 5.0);
        assert_eq!(mat[1][0], 7.0);
        assert_eq!(mat[1][1], 2.0);
    }
}