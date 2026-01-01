use std::ops::{Add, Div, Index, Mul, Neg, Sub};

use crate::{matrix::{matrix::Matrix, rotation::{Angle, Rotation}, scale::Scale}, vector::vector3::Vector3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3 {
    pub x: Vector3,
    pub y: Vector3,
    pub z: Vector3,
}

impl Matrix3 {
    pub fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) -> Self {
        let x = Vector3::new(m00, m01, m02);
        let y = Vector3::new(m10, m11, m12);
        let z = Vector3::new(m20, m21, m22);
        Matrix3::from_rows(x, y, z)
    }

    fn from_rows(x: Vector3, y: Vector3, z: Vector3) -> Self {
        Self {
            x: Vector3::new(x[0], y[0], z[0]),
            y: Vector3::new(x[1], y[1], z[1]),
            z: Vector3::new(x[2], y[2], z[2]),
        }
    }

    pub fn from_cols(x: Vector3, y: Vector3, z: Vector3) -> Self {
        Self { x, y, z }
    }
}

impl Matrix for Matrix3 {
    type Vector = Vector3;

    fn row(&self, index: usize) -> Vector3 {
        match index {
            0 => Vector3::new(self.x[0], self.y[0], self.z[0]),
            1 => Vector3::new(self.x[1], self.y[1], self.z[1]),
            2 => Vector3::new(self.x[2], self.y[2], self.z[2]),
            _ => panic!("Out of range"),
        }
    }

    fn col(&self, index: usize) -> Vector3 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Out of range"),
        }
    }

    fn transpose(&self) -> Matrix3 {
        Matrix3::from_rows(self.x, self.y, self.z)
    }
    
    fn identity() -> Self {
        let x = Vector3::new(1, 0, 0);
        let y = Vector3::new(0, 1, 0);
        let z = Vector3::new(0, 0, 1);
        Matrix3::from_cols(x, y, z)
    }
}
impl Scale for Matrix3 {
    type Output = Matrix3;

    fn scalar_matrix(scalar: f32) -> Matrix3 {
        scalar * Matrix3::identity()
    }
}

impl Rotation for Matrix3 {
    type Output = Matrix3;

    fn x_rotation_matrix(angle: Angle) -> Matrix3 {
        let angle: f32 = match angle {
            Angle::Degrees(degrees) => degrees.to_radians(),
            Angle::Radians(radians) => radians
        };

        let cos: f32 = f32::cos(angle);
        let sin: f32 = f32::sin(angle);

        Matrix3::new(
            1.0, 0.0, 0.0, 
            0.0, cos, -sin, 
            0.0, sin, cos
        )
    }


    fn y_rotation_matrix(angle: Angle) -> Matrix3 {
        let angle: f32 = match angle {
            Angle::Degrees(degrees) => degrees.to_radians(),
            Angle::Radians(radians) => radians
        };

        let cos: f32 = f32::cos(angle);
        let sin: f32 = f32::sin(angle);

        Matrix3::new(
            cos, 0.0, sin,
            0.0, 1.0, 0.0,
            -sin, 0.0, cos
        )
    }

    fn z_rotation_matrix(angle: Angle) -> Matrix3 {
        let angle: f32 = match angle {
            Angle::Degrees(degrees) => degrees.to_radians(),
            Angle::Radians(radians) => radians
        };

        let cos: f32 = f32::cos(angle);
        let sin: f32 = f32::sin(angle);

        Matrix3::new(
            cos, -sin, 0.0,
            sin, cos, 0.0,
            0.0, 0.0, 1.0
        )
    }

    fn rotation_matrix(yaw: Angle, pitch: Angle, roll: Angle) -> Matrix3 {
        Matrix3::z_rotation_matrix(roll) * 
        Matrix3::y_rotation_matrix(pitch) * 
        Matrix3::x_rotation_matrix(yaw) 
    }
}

// Matrix-Vector Multiplication
impl Mul<Vector3> for Matrix3 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Self::Output {
        v.x * self.x + v.y * self.y + v.z * self.z
    }
}

// Matrix multiplication
impl Mul<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, other: Matrix3) -> Self::Output {
        let x = self * other.x;
        let y = self * other.y;
        let z = self * other.z;
        Matrix3::from_cols(x, y, z)
    }
}

// Scalar - Matrix multiplication
impl Mul<f32> for Matrix3 {
    type Output = Matrix3;

    fn mul(self, scalar: f32) -> Self::Output {
        let x = scalar * self.x;
        let y = scalar * self.y;
        let z = scalar * self.z;
        Matrix3::from_cols(x, y, z)
    }
}

impl Mul<Matrix3> for f32 {
    type Output = Matrix3;

    fn mul(self, mat: Matrix3) -> Self::Output {
        let x = self * mat.x;
        let y = self * mat.y;
        let z = self * mat.z;
        Matrix3::from_cols(x, y, z)
    }
}

// Scalar-Matrix multiplication
impl Div<f32> for Matrix3 {
    type Output = Matrix3;

    fn div(self, scalar: f32) -> Self::Output {
        let x = self.x / scalar;
        let y = self.y / scalar;
        let z = self.z / scalar;
        Matrix3::from_cols(x, y, z)
    }
}

impl Div<Matrix3> for f32 {
    type Output = Matrix3;

    fn div(self, mat: Matrix3) -> Self::Output {
        let x = self / mat.x;
        let y = self / mat.y;
        let z = self / mat.z;
        Matrix3::from_cols(x, y, z)
    }
}

// Scalar-Matrix Addition
impl Add<f32> for Matrix3 {
    type Output = Matrix3;

    fn add(self, scalar: f32) -> Self::Output {
        let x = scalar + self.x;
        let y = scalar + self.y;
        let z = scalar + self.z;
        Matrix3::from_cols(x, y, z)
    }
}

impl Add<Matrix3> for f32 {
    type Output = Matrix3;

    fn add(self, mat: Matrix3) -> Self::Output {
        let x = mat.x + self;
        let y = mat.y + self;
        let z = mat.z + self;
        Matrix3::from_cols(x, y, z)
    }
}

// Matrix subtraction
impl Sub<f32> for Matrix3 {
    type Output = Matrix3;

    fn sub(self, scalar: f32) -> Self::Output {
        let x = self.x - scalar;
        let y = self.y - scalar;
        let z = self.z - scalar;
        Matrix3::from_cols(x, y, z)
    }
}

impl Sub<Matrix3> for f32 {
    type Output = Matrix3;

    fn sub(self, mat: Matrix3) -> Self::Output {
        let x = self - mat.x;
        let y = self - mat.y;
        let z = self - mat.z;
        Matrix3::from_cols(x, y, z)
    }
}

// Matrix Addition
impl Add<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn add(self, other: Matrix3) -> Self::Output {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Matrix3::from_cols(x, y, z)
    }
}

// Matrix Subtraction
impl Sub<Matrix3> for Matrix3 {
    type Output = Matrix3;

    fn sub(self, other: Matrix3) -> Self::Output {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Matrix3::from_cols(x, y, z)
    }
}

// Matrix negation
impl Neg for Matrix3 {
    type Output = Matrix3;

    fn neg(self) -> Matrix3 {
        Matrix3::from_cols(-self.x, -self.y, -self.z)
    }
}

impl Index<usize> for Matrix3 {
    type Output = Vector3;

    fn index(&self, index: usize) -> &Vector3 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range"),
        }
    }
}


mod tests {
    use crate::matrix::matrix2::Matrix2;

    use super::*;

    #[test]
    fn indexing_test() {
        let x = Vector3::new(10, 5, 2);
        let y = Vector3::new(7, 2, 1);
        let z = Vector3::new(12, 9, 3);
        let mat = Matrix3::from_cols(x, y, z);

        // First index (column), second index (Row)
        assert_eq!(mat[0][0], 10.0);
        assert_eq!(mat[0][1], 5.0);
        assert_eq!(mat[0][2], 2.0);
        assert_eq!(mat[1][0], 7.0);
        assert_eq!(mat[1][1], 2.0);
        assert_eq!(mat[1][2], 1.0);
        assert_eq!(mat[2][0], 12.0);
        assert_eq!(mat[2][1], 9.0);
        assert_eq!(mat[2][2], 3.0);
    }

    #[test]
    fn transpose_test() {
        let a = Matrix3::from_rows(
            Vector3::new(10, 2, 3),
            Vector3::new(5, 12, 11),
            Vector3::new(9, 1, 4),
        );
        let t = Matrix3::from_rows(
            Vector3::new(10, 5, 9),
            Vector3::new(2, 12, 1),
            Vector3::new(3, 11, 4),
        );
        assert_eq!(a.transpose(), t)
    }

    #[test]
    fn matrix_vector_multiplication_test() {
        let a = Matrix3::from_rows(
            Vector3::new(10, 2, 3),
            Vector3::new(5, 12, 11),
            Vector3::new(9, 1, 4),
        );
        let b = Vector3::new(2, 3, 4);
        let res = Vector3::new(38, 90, 37);
        assert_eq!(a * b, res);
    }

    #[test]
    fn matrix_multiplication_test() {
        let a = Matrix3::from_rows(
            Vector3::new(10, 2, 3),
            Vector3::new(5, 12, 11),
            Vector3::new(9, 1, 4),
        );
        let b = Matrix3::from_rows(
            Vector3::new(31, 132, 12),
            Vector3::new(1, 99, 119),
            Vector3::new(23, 34, 71),
        );
        let res = Matrix3::from_rows(
            Vector3::new(381, 1620, 571),
            Vector3::new(420, 2222, 2269),
            Vector3::new(372, 1423, 511),
        );
        assert_eq!(a * b, res);
    }

    #[test]
    fn matrix_scalar_multiplication_test() {
        let a = Matrix3::from_rows(
            Vector3::new(10, 2, 3),
            Vector3::new(5, 12, 11),
            Vector3::new(9, 1, 4),
        );
        let res = Matrix3::from_rows(
            Vector3::new(20, 4, 6),
            Vector3::new(10, 24, 22),
            Vector3::new(18, 2, 8),
        );
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
        let a = Matrix3::new(
            1.0, 2.0, 3.0, 
            2.1, 12.0, 29.0,
            11.1, 3.0, 123.5,
        );
        let res = Matrix3::new(
            3.0, 4.0, 5.0, 
            4.1, 14.0, 31.0,
            13.1, 5.0, 125.5,
        );
        assert_eq!(a + 2., res);
        assert_eq!(2. + a, res);
    }

    #[test]
    fn matrix_scalar_subtraction_test() {
        let a = Matrix3::new(
            1.0, 2.0, 3.0, 
            5.0, 12.0, 29.0,
            11.1, 3.0, 123.5,
        );
        let res = Matrix3::new(
            -1.0, 0.0, 1.0, 
            3.0, 10.0, 27.0,
            9.1, 1.0, 121.5,
        );
        assert_eq!(a - 2., res);
        assert_eq!(2. - a, -res);
    }

    #[test]
    fn matrix_addition_test() {
        let a = Matrix3::new(
            1.0, 2.0, 3.0, 
            2.11, 12.0, 29.0,
            11.1, 3.0, 123.5,
        );
        let b = Matrix3::new(
            5.5, 1.1, 6.0, 
            9.5, 111.0, 74.0,
            81.1, 99.0, -2.0,
        );
        let res = Matrix3::new(
            6.5, 3.1, 9.0, 
            11.61, 123.0, 103.0,
            92.2, 102.0, 121.5,
        );
        assert_eq!(a + b, res);
        assert_eq!(b + a, res);
    }

    #[test]
    fn matrix_subtraction_test() {
        let a = Matrix3::new(
            1.0, 2.0, 3.0, 
            2.1, 12.0, 29.0,
            11.1, 3.0, 123.5,
        );
        let b = Matrix3::new(
            5.5, 1.1, 6.0, 
            9.5, 111.0, 74.0,
            81.1, 99.0, -2.0,
        );
        let res = Matrix3::new(
            -4.5, 0.9, -3.0, 
            -7.4, -99.0, -45.0,
            -70.0, -96.0, 125.5,
        );
        assert_eq!(a - b, res);
        assert_eq!(b - a, -res);
    }
}
