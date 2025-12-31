use std::ops::{self, Add, Div, Index, Mul, Neg, Sub};

use num::{ToPrimitive, pow};

use crate::{Vector2, vector::vector::Vector};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {
    pub fn new<T, U, V>(x: T, y: U, z: V) -> Vector3
    where
        T: ToPrimitive,
        U: ToPrimitive,
        V: ToPrimitive,
    {
        Vector3 {
            x: x.to_f32().expect("Not a number"),
            y: y.to_f32().expect("Not a number"),
            z: z.to_f32().expect("Not a number"),
        }
    }

    fn cross(&self, other: Vector3) -> Vector3 {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Vector for Vector3 {
    type VectorType = Vector3;

    fn length(&self) -> f32 {
        f32::sqrt(pow(self.x, 2) + pow(self.y, 2) + pow(self.z, 2))
    }

    fn normalize(&self) -> Self::VectorType {
        let length = self.length();
        *self / length
    }

    fn dot(&self, other: Self::VectorType) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

// Vector addition
impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3::new(other.x + self.x, other.y + self.y, other.z + self.z)
    }
}

// Vector subtraction
impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

// Scalar-vector addition
impl Add<f32> for Vector3 {
    type Output = Vector3;
    fn add(self, scalar: f32) -> Vector3 {
        Vector3::new(scalar + self.x, scalar + self.y, scalar + self.z)
    }
}

impl Add<Vector3> for f32 {
    type Output = Vector3;
    fn add(self, vec: Vector3) -> Vector3 {
        Vector3::new(self + vec.x, self + vec.y, self + vec.y)
    }
}

// Scalar-vector subtraction
impl Sub<f32> for Vector3 {
    type Output = Vector3;

    fn sub(self, scalar: f32) -> Vector3 {
        Vector3::new(self.x - scalar, self.y - scalar, self.z - scalar)
    }
}

impl Sub<Vector3> for f32 {
    type Output = Vector3;
    fn sub(self, vec: Vector3) -> Vector3 {
        Vector3::new(self - vec.x, self - vec.y, self - vec.z)
    }
}

// Hadamard product
impl Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

// Scalar-Vector multiplication
impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, scalar: f32) -> Vector3 {
        Vector3::new(scalar * self.x, scalar * self.y, scalar * self.z)
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;
    fn mul(self, vec: Vector3) -> Vector3 {
        Vector3::new(self * vec.x, self * vec.y, self * vec.z)
    }
}

// Scalar-Vector division
impl Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, scalar: f32) -> Vector3 {
        Vector3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl Div<Vector3> for f32 {
    type Output = Vector3;
    fn div(self, vec: Vector3) -> Vector3 {
        Vector3::new(self / vec.x, self / vec.y, self / vec.z)
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}


// Allows us to index instead of using members
impl Index<usize> for Vector3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of range"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_multiplication_test() {
        let a = Vector3::new(5, 10, 10);
        let res = Vector3::new(10, 20, 20);

        assert_eq!(a * 2., res);
        assert_eq!(2. * a, res);
    }

    #[test]
    fn normalize_test() {
        let a: Vector3 = Vector3::new(10, 5, 20);
        let b: Vector3 = Vector3::new(
            2. / f32::sqrt(21.),
            1. / f32::sqrt(21.),
            4. / f32::sqrt(21.),
        );

        assert!((a.normalize() - b).length() <= 0.001);
    }

    #[test]
    fn scalar_subtraction_test() {
        let vec: Vector3 = Vector3::new(10., 20., 5.);
        let res: Vector3 = Vector3::new(8, 18, 3);
        assert_eq!(2. - vec, -res);
        assert_eq!(vec - 2., res);
    }

    #[test]
    fn negation_test() {
        let vec: Vector3 = Vector3::new(10., 20., 5.);
        let res: Vector3 = Vector3::new(-10., -20., -5.);
        assert_eq!(-vec, res);
    }

    #[test]
    fn vector_addition_test() {
        let a: Vector3 = Vector3::new(10., 39., 29.);
        let b: Vector3 = Vector3::new(2., 520., 25.);
        let res: Vector3 = Vector3::new(12, 559, 54);
        assert_eq!(a + b, res);
        assert_eq!(b + a, res);
    }

    #[test]
    fn vector_subtraction_test() {
        let a: Vector3 = Vector3::new(10., 39., 23.);
        let b: Vector3 = Vector3::new(3., 519., 4.);
        let res: Vector3 = Vector3::new(7, -480, 19);
        assert_eq!(a - b, res);
        assert_eq!(b - a, -res);
    }


    #[test]
    fn magnitude_test() {
        let a: Vector3 = Vector3::new(10., 18., 2.);
        assert_eq!(a.length(), f32::sqrt(428.));
    }

    #[test]
    fn dot_product_test() {
        let a = Vector3::new(1, 2, 3);
        let b = Vector3::new(3, 4, 5);
        let res = 26.0;

        assert_eq!(a.dot(b), res);
    }

    #[test]
    fn cross_product_test() {
        let a = Vector3::new(1, 2, 3);
        let b = Vector3::new(12, 4, 5);
        let res = Vector3::new(-2, 31, -20);

        assert_eq!(a.cross(b), res);
    }
}
