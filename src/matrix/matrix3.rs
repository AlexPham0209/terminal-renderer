use std::ops::{Add, Index, Mul, Sub};

use crate::{matrix::matrix::Matrix, vector::vector3::Vector3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3 {
    x: Vector3,
    y: Vector3,
    z: Vector3,
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

// Matrix - Vector Multiplication
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

// Scalar - Matrix Addition
impl Add<f32> for Matrix3 {
    type Output = Matrix3;

    fn add(self, scalar: f32) -> Self::Output {
        let x = scalar + self.x;
        let y = scalar + self.y;
        let z = scalar + self.z;
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
