use std::ops::{Add, Index, Mul, Sub};

use crate::{matrix::matrix::Matrix, vector::vector4::Vector4};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4 {
    x: Vector4,
    y: Vector4,
    z: Vector4,
    w: Vector4,
}

impl Matrix4 {
    pub fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m03: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m13: f32,
        m20: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m30: f32,
        m31: f32,
        m32: f32,
        m33: f32,
    ) -> Self {
        let x = Vector4::new(m00, m01, m02, m03);
        let y = Vector4::new(m10, m11, m12, m13);
        let z = Vector4::new(m20, m21, m22, m23);
        let w = Vector4::new(m30, m31, m32, m33);
        Matrix4::from_rows(x, y, z, w)
    }

    fn from_rows(x: Vector4, y: Vector4, z: Vector4, w: Vector4) -> Self {
        Self {
            x: Vector4::new(x[0], y[0], z[0], w[0]),
            y: Vector4::new(x[1], y[1], z[1], w[1]),
            z: Vector4::new(x[2], y[2], z[2], w[2]),
            w: Vector4::new(x[3], y[3], z[3], w[3]),
        }
    }

    pub fn from_cols(x: Vector4, y: Vector4, z: Vector4, w: Vector4) -> Self {
        Self { x, y, z, w }
    }
}

impl Matrix for Matrix4 {
    type Vector = Vector4;

    fn row(&self, index: usize) -> Vector4 {
        match index {
            0 => Vector4::new(self.x[0], self.y[0], self.z[0], self.w[0]),
            1 => Vector4::new(self.x[1], self.y[1], self.z[1], self.w[1]),
            2 => Vector4::new(self.x[2], self.y[2], self.z[2], self.w[2]),
            3 => Vector4::new(self.x[3], self.y[3], self.z[3], self.w[3]),
            _ => panic!("Out of range"),
        }
    }

    fn col(&self, index: usize) -> Vector4 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Out of range"),
        }
    }

    fn transpose(&self) -> Matrix4 {
        Matrix4::from_rows(self.x, self.y, self.z, self.w)
    }
}

impl Index<usize> for Matrix4 {
    type Output = Vector4;

    fn index(&self, index: usize) -> &Vector4 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of range"),
        }
    }
}

// Matrix-Vector Multiplication
impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, v: Vector4) -> Self::Output {
        v.x * self.x + v.y * self.y + v.z * self.z + v.w * self.w
    }
}

// Matrix multiplication
impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, other: Matrix4) -> Self::Output {
        let x = self * other.x;
        let y = self * other.y;
        let z = self * other.z;
        let w = self * other.w;
        Matrix4::from_cols(x, y, z, w)
    }
}

// Scalar - Matrix multiplication
impl Mul<f32> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, scalar: f32) -> Self::Output {
        let x = scalar * self.x;
        let y = scalar * self.y;
        let z = scalar * self.z;
        let w = scalar * self.w;
        Matrix4::from_cols(x, y, z, w)
    }
}

// Scalar - Matrix Addition
impl Add<f32> for Matrix4 {
    type Output = Matrix4;

    fn add(self, scalar: f32) -> Self::Output {
        let x = scalar + self.x;
        let y = scalar + self.y;
        let z = scalar + self.z;
        let w = scalar + self.w;
        Matrix4::from_cols(x, y, z, w)
    }
}

// Matrix subtraction
impl Sub<f32> for Matrix4 {
    type Output = Matrix4;

    fn sub(self, scalar: f32) -> Self::Output {
        let x = self.x - scalar;
        let y = self.y - scalar;
        let z = self.z - scalar;
        let w = self.w - scalar;
        Matrix4::from_cols(x, y, z, w)
    }
}

impl Sub<Matrix4> for f32 {
    type Output = Matrix4;

    fn sub(self, mat: Matrix4) -> Self::Output {
        let x = self - mat.x;
        let y = self - mat.y;
        let z = self - mat.z;
        let w = self - mat.w;
        Matrix4::from_cols(x, y, z, w)
    }
}

// Matrix Addition
impl Add<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn add(self, other: Matrix4) -> Self::Output {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        let w = self.w + other.w;
        Matrix4::from_cols(x, y, z, w)
    }
}

// Matrix Subtraction
impl Sub<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn sub(self, other: Matrix4) -> Self::Output {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        let w = self.w - other.w;
        Matrix4::from_cols(x, y, z, w)
    }
}
