use std::ops::{Add, Div, Index, Mul, Neg, Sub};

use crate::{Vector2, matrix::matrix::Matrix, vector::vector3::Vector3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2 {
    x: Vector2,
    y: Vector2,
}

impl Matrix2 {
    pub fn new(m00: f32, m01: f32, m10: f32, m11: f32) -> Self {
        let x = Vector2::new(m00, m01);
        let y = Vector2::new(m10, m11);
        Matrix2::from_rows(x, y)
    }

    fn from_rows(x: Vector2, y: Vector2) -> Self {
        Self {
            x: Vector2::new(x[0], y[0]),
            y: Vector2::new(x[1], y[1]),
        }
    }

    pub fn from_cols(x: Vector2, y: Vector2) -> Self {
        Self { x, y }
    }
}

impl Matrix for Matrix2 {
    type Vector = Vector2;

    fn row(&self, index: usize) -> Vector2 {
        match index {
            0 => Vector2::new(self.x[0], self.y[0]),
            1 => Vector2::new(self.x[1], self.y[1]),
            _ => panic!("Out of range"),
        }
    }

    fn col(&self, index: usize) -> Vector2 {
        match index {
            0 => self.x,
            1 => self.y,
            _ => panic!("Out of range"),
        }
    }

    fn transpose(&self) -> Matrix2 {
        Matrix2::from_rows(self.x, self.y)
    }
}

impl Index<usize> for Matrix2 {
    type Output = Vector2;

    fn index(&self, index: usize) -> &Vector2 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of range"),
        }
    }
}

// Matrix-Vector Multiplication
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

// Scalar-Matrix multiplication
impl Mul<f32> for Matrix2 {
    type Output = Matrix2;

    fn mul(self, scalar: f32) -> Self::Output {
        let x = scalar * self.x;
        let y = scalar * self.y;
        Matrix2::from_cols(x, y)
    }
}

impl Mul<Matrix2> for f32 {
    type Output = Matrix2;

    fn mul(self, mat: Matrix2) -> Self::Output {
        let x = self * mat.x;
        let y = self * mat.y;
        Matrix2::from_cols(x, y)
    }
}

// Scalar-Matrix multiplication
impl Div<f32> for Matrix2 {
    type Output = Matrix2;

    fn div(self, scalar: f32) -> Self::Output {
        let x = self.x / scalar;
        let y = self.y / scalar;
        Matrix2::from_cols(x, y)
    }
}

impl Div<Matrix2> for f32 {
    type Output = Matrix2;

    fn div(self, mat: Matrix2) -> Self::Output {
        let x = self / mat.x;
        let y = self / mat.y;
        Matrix2::from_cols(x, y)
    }
}

// Scalar-Matrix Addition
impl Add<f32> for Matrix2 {
    type Output = Matrix2;

    fn add(self, scalar: f32) -> Self::Output {
        let x = scalar + self.x;
        let y = scalar + self.y;
        Matrix2::from_cols(x, y)
    }
}

impl Add<Matrix2> for f32 {
    type Output = Matrix2;

    fn add(self, mat: Matrix2) -> Self::Output {
        let x = mat.x + self;
        let y = mat.y + self;
        Matrix2::from_cols(x, y)
    }
}

// Scalar-Matrix Subtraction
impl Sub<f32> for Matrix2 {
    type Output = Matrix2;

    fn sub(self, scalar: f32) -> Self::Output {
        let x = self.x - scalar;
        let y = self.y - scalar;
        Matrix2::from_cols(x, y)
    }
}

impl Sub<Matrix2> for f32 {
    type Output = Matrix2;

    fn sub(self, mat: Matrix2) -> Self::Output {
        let x = self - mat.x;
        let y = self - mat.y;
        Matrix2::from_cols(x, y)
    }
}

// Matrix Addition
impl Add<Matrix2> for Matrix2 {
    type Output = Matrix2;

    fn add(self, other: Matrix2) -> Self::Output {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Matrix2::from_cols(x, y)
    }
}

// Matrix subtraction
impl Sub<Matrix2> for Matrix2 {
    type Output = Matrix2;

    fn sub(self, other: Matrix2) -> Self::Output {
        let x = self.x - other.x;
        let y = self.y - other.y;
        Matrix2::from_cols(x, y)
    }
}

// Matrix negation
impl Neg for Matrix2 {
    type Output = Matrix2;

    fn neg(self) -> Matrix2 {
        Matrix2::from_cols(-self.x, -self.y)
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
    fn transpose_test() {
        let a = Matrix2::new(1.0, 2.0, 3.0, 4.0);
        let t = Matrix2::new(1.0, 3.0, 2.0, 4.0);
        assert_eq!(a.transpose(), t)
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

    #[test]
    fn matrix_scalar_multiplication_test() {
        let a = Matrix2::new(1.0, 2.0, 3.0, 2.0);
        let res = Matrix2::new(2.0, 4.0, 6.0, 4.0);
        assert_eq!(a * 2., res);
        assert_eq!(2. * a, res);
    }

    #[test]
    fn matrix_scalar_division_test() {
        let a = Matrix2::new(2.0, 4.0, 6.0, 4.0);
        let b = Matrix2::new(1.0, 2.0, 3.0, 2.0);
        let c = Matrix2::new(1.0, 0.5, 0.33333334, 0.5);
        assert_eq!(a / 2., b);
        assert_eq!(2. / a, c);
    }

    #[test]
    fn matrix_scalar_addition_test() {
        let a = Matrix2::new(1.0, 2.0, 3.0, 2.0);
        let res = Matrix2::new(3.0, 4.0, 5.0, 4.0);
        assert_eq!(a + 2., res);
        assert_eq!(2. + a, res);
    }

    #[test]
    fn matrix_scalar_subtraction_test() {
        let a = Matrix2::new(1.0, 2.0, 3.0, 2.0);
        let res: Matrix2 = Matrix2::new(-1.0, 0.0, 1.0, 0.0);
        assert_eq!(a - 2., res);
        assert_eq!(2. - a, -res);
    }

    #[test]
    fn matrix_addition_test() {
        let a = Matrix2::new(1.0, 2.0, 3.0, 2.0);
        let b =  Matrix2::new(1.5, 12.0, 5.0, -2.0);
        let res: Matrix2 = Matrix2::new(2.5, 14.0, 8.0, 0.0);
        assert_eq!(a + b, res);
        assert_eq!(b + a, res);
    }

    #[test]
    fn matrix_subtraction_test() {
        let a = Matrix2::new(1.0, 2.0, 3.0, 2.0);
        let b =  Matrix2::new(1.5, 12.0, 5.0, -2.0);
        let res: Matrix2 = Matrix2::new(-0.5, -10.0, -2.0, 4.0);
        assert_eq!(a - b, res);
        assert_eq!(b - a, -res);
    }
}
