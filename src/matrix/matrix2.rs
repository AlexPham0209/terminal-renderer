use std::ops::{Index, Mul};

use crate::{Vector2, vector::vector3::Vector3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2 {
    x: Vector2,
    y: Vector2,
}

impl Matrix2 {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        let x = Vector2::new(a, b);
        let y = Vector2::new(c, d);
        Matrix2::from_rows(x, y)
    }
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

// Matrix - Vector Multiplication
impl Mul<Vector2> for Matrix2 {
    type Output = Vector2;

    fn mul(self, v: Vector2) -> Self::Output {
        v.x * self.x + v.y * self.y
    }
}

// Matrix multiplication
impl Mul<Matrix2> for Matrix2 {
    type Output = Matrix2;

    fn mul(self, other: Matrix2) -> Self::Output {
        let x = self * other.x;
        let y = self * other.y;
        Matrix2::from_cols(x, y)
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

    #[test]
    fn matrix_vector_multiplication_test() {
        let a = Matrix2::new(1.0, 2.0, 3.0, 2.0);
        let b = Vector2::new(4, 5);
        let res = Vector2::new(14, 22);
        assert_eq!(a * b, res);
    }

    #[test]
    fn matrix_multiplication_test() {
        let a = Matrix2::new(1.0, 2.0, 3.0, 2.0);
        let b = Matrix2::new(3.0, 2.0, 1.0, 5.0);
        let res = Matrix2::new(5.0, 12.0, 11.0, 16.0);
        assert_eq!(a * b, res);
    }
}