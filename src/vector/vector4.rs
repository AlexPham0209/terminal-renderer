use std::ops::{self, Add, Index, Mul, Neg, Sub};

use num::{ToPrimitive, pow};

use crate::{
    Vector2,
    vector::{vector::Vector, vector3::Vector3},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new<T, U, V, W>(x: T, y: U, z: V, w: W) -> Vector4
    where
        T: ToPrimitive,
        U: ToPrimitive,
        V: ToPrimitive,
        W: ToPrimitive,
    {
        Vector4 {
            x: x.to_f32().expect("Not a number"),
            y: y.to_f32().expect("Not a number"),
            z: z.to_f32().expect("Not a number"),
            w: w.to_f32().expect("Not a number"),
        }
    }

    pub fn to_homogenous(v: Vector3) -> Vector4 {
        let Vector3 { x, y, z } = v;
        Vector4::new(x, y, z, 1.0)
    }
}

impl Vector for Vector4 {
    type VectorType = Vector4;

    fn length(&self) -> f32 {
        f32::sqrt(pow(self.x, 2) + pow(self.y, 2) + pow(self.z, 2) + pow(self.w, 2))
    }

    fn normalize(&self) -> Self::VectorType {
        let length = self.length();
        *self * (1. / length)
    }

    fn dot(&self, other: &Self::VectorType) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

// Vector addition
impl Add<Vector4> for Vector4 {
    type Output = Vector4;

    fn add(self, other: Vector4) -> Vector4 {
        Vector4::new(
            other.x + self.x,
            other.y + self.y,
            other.z + self.z,
            other.w + self.w,
        )
    }
}

// Vector subtraction
impl Sub<Vector4> for Vector4 {
    type Output = Vector4;

    fn sub(self, other: Vector4) -> Vector4 {
        Vector4::new(
            other.x - self.x,
            other.y - self.y,
            other.z - self.z,
            other.w - self.w,
        )
    }
}

// Scalar-vector addition
impl Add<f32> for Vector4 {
    type Output = Vector4;
    fn add(self, scalar: f32) -> Vector4 {
        Vector4::new(
            scalar + self.x,
            scalar + self.y,
            scalar + self.z,
            scalar + self.w,
        )
    }
}

impl Add<Vector4> for f32 {
    type Output = Vector4;
    fn add(self, vec: Vector4) -> Vector4 {
        Vector4::new(self + vec.x, self + vec.y, self + vec.y, self + vec.w)
    }
}

// Scalar-vector subtraction
impl Sub<f32> for Vector4 {
    type Output = Vector4;

    fn sub(self, scalar: f32) -> Vector4 {
        Vector4::new(self.x - scalar, self.y - scalar, self.z - scalar, self.w - scalar)
    }
}

impl Sub<Vector4> for f32 {
    type Output = Vector4;
    fn sub(self, vec: Vector4) -> Vector4 {
        Vector4::new(self - vec.x, self - vec.y, self - vec.z, self - vec.w)
    }
}

impl Neg for Vector4 {
    type Output = Vector4;

    fn neg(self) -> Vector4 {
        Vector4::new(-self.x, -self.y, -self.z, -self.w)
    }
}

// Hadamard product
impl Mul<Vector4> for Vector4 {
    type Output = Vector4;
    fn mul(self, other: Vector4) -> Vector4 {
        Vector4::new(
            self.x * other.x,
            self.y * other.y,
            self.z * other.z,
            self.w * other.w,
        )
    }
}

// Scalar-Vector multiplication
impl Mul<f32> for Vector4 {
    type Output = Vector4;
    fn mul(self, scalar: f32) -> Vector4 {
        Vector4::new(
            scalar * self.x,
            scalar * self.y,
            scalar * self.z,
            scalar * self.w,
        )
    }
}

impl Mul<Vector4> for f32 {
    type Output = Vector4;
    fn mul(self, vec: Vector4) -> Vector4 {
        Vector4::new(self * vec.x, self * vec.y, self * vec.z, self * vec.w)
    }
}

// Allows us to index instead of using members
impl Index<usize> for Vector4 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of range"),
        }
    }
}
