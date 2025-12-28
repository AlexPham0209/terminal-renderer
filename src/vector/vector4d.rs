use std::ops;

use num::{ToPrimitive, pow};

use crate::vector::vector::Vector;

#[derive(Clone, Copy, Debug)]
pub struct Vector4D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4D {
    pub fn new<T, U, V, W>(x: T, y: U, z: V, w: W) -> Vector4D
    where
        T: ToPrimitive,
        U: ToPrimitive,
        V: ToPrimitive,
        W: ToPrimitive,
    {
        Vector4D {
            x: x.to_f32().expect("Not a number"),
            y: y.to_f32().expect("Not a number"),
            z: z.to_f32().expect("Not a number"),
            w: w.to_f32().expect("Not a number"),
        }
    }
}


impl Vector for Vector4D {
    type VectorType = Vector4D;

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
impl ops::Add<Vector4D> for Vector4D {
    type Output = Vector4D;

    fn add(self, other: Vector4D) -> Vector4D {
        Vector4D::new(other.x + self.x, other.y + self.y, other.z + self.z, other.w + self.w)
    }
}

// Scalar-vector addition
impl ops::Add<f32> for Vector4D {
    type Output = Vector4D;
    fn add(self, scalar: f32) -> Vector4D {
        Vector4D::new(scalar + self.x, scalar + self.y, scalar + self.z, scalar + self.w)
    }
}

impl ops::Add<Vector4D> for f32 {
    type Output = Vector4D;
    fn add(self, vec: Vector4D) -> Vector4D {
        Vector4D::new(self + vec.x, self + vec.y, self + vec.y, self + vec.w)
    }
}

// Hadamard product
impl ops::Mul<Vector4D> for Vector4D {
    type Output = Vector4D;
    fn mul(self, other: Vector4D) -> Vector4D {
        Vector4D::new(self.x * other.x, self.y * other.y, self.z * other.z, self.w * other.w)
    }
}

// Scalar-Vector multiplication
impl ops::Mul<f32> for Vector4D {
    type Output = Vector4D;
    fn mul(self, scalar: f32) -> Vector4D {
        Vector4D::new(scalar * self.x, scalar * self.y, scalar * self.z, scalar * self.w)
    }
}

impl ops::Mul<Vector4D> for f32 {
    type Output = Vector4D;
    fn mul(self, vec: Vector4D) -> Vector4D {
        Vector4D::new(self * vec.x, self * vec.y, self * vec.z, self * vec.w)
    }
}
