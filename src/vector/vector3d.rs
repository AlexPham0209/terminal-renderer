use std::ops;

use num::{ToPrimitive, pow};

use crate::{
    Vector2D,
    vector::vector::{CrossProduct, Vector},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3D {
    pub fn new<T, U, V>(x: T, y: U, z: V) -> Vector3D
    where
        T: ToPrimitive,
        U: ToPrimitive,
        V: ToPrimitive,
    {
        Vector3D {
            x: x.to_f32().expect("Not a number"),
            y: y.to_f32().expect("Not a number"),
            z: z.to_f32().expect("Not a number"),
        }
    }
}

impl Vector for Vector3D {
    type VectorType = Vector3D;

    fn length(&self) -> f32 {
        f32::sqrt(pow(self.x, 2) + pow(self.y, 2) + pow(self.z, 2))
    }

    fn normalize(&self) -> Self::VectorType {
        let length = self.length();
        *self * (1. / length)
    }

    fn dot(&self, other: &Self::VectorType) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl CrossProduct for Vector3D {
    type VectorType = Vector3D;
    type Output = Vector3D;

    fn cross(&self, other: &Self::VectorType) -> Self::Output {
        Vector3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

// Vector addition
impl ops::Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, other: Vector3D) -> Vector3D {
        Vector3D::new(other.x + self.x, other.y + self.y, other.z + self.z)
    }
}

// Scalar-vector addition
impl ops::Add<f32> for Vector3D {
    type Output = Vector3D;
    fn add(self, scalar: f32) -> Vector3D {
        Vector3D::new(scalar + self.x, scalar + self.y, scalar + self.z)
    }
}

impl ops::Add<Vector3D> for f32 {
    type Output = Vector3D;
    fn add(self, vec: Vector3D) -> Vector3D {
        Vector3D::new(self + vec.x, self + vec.y, self + vec.y)
    }
}

// Hadamard product
impl ops::Mul<Vector3D> for Vector3D {
    type Output = Vector3D;
    fn mul(self, other: Vector3D) -> Vector3D {
        Vector3D::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

// Scalar-Vector multiplication
impl ops::Mul<f32> for Vector3D {
    type Output = Vector3D;
    fn mul(self, scalar: f32) -> Vector3D {
        Vector3D::new(scalar * self.x, scalar * self.y, scalar * self.z)
    }
}

impl ops::Mul<Vector3D> for f32 {
    type Output = Vector3D;
    fn mul(self, vec: Vector3D) -> Vector3D {
        Vector3D::new(self * vec.x, self * vec.y, self * vec.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_multiplication_test() {
        let a = Vector3D::new(5, 10, 10);
        let res = Vector3D::new(10, 20, 20);

        assert_eq!(a * 2., res);
        assert_eq!(2. * a, res);
    }
}
