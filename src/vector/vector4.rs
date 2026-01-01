use std::ops::{self, Add, Div, Index, Mul, Neg, Sub};

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

    pub fn to_homogeneous(v: Vector3) -> Vector4 {
        Vector4::from_vector3(v, 1.0)
    }
    
    pub fn from_vector3(v: Vector3, w: f32) -> Vector4 {
        let Vector3 { x, y, z } = v;
        Vector4::new(x, y, z, w)
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

    fn dot(&self, other: Self::VectorType) -> f32 {
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
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
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
        Vector4::new(self + vec.x, self + vec.y, self + vec.z, self + vec.w)
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

// Scalar-vector division
impl Div<f32> for Vector4 {
    type Output = Vector4;
    fn div(self, scalar: f32) -> Vector4 {
        Vector4::new(self.x / scalar, self.y / scalar, self.z / scalar, self.w / scalar)
    }
}

impl Div<Vector4> for f32 {
    type Output = Vector4;
    fn div(self, vec: Vector4) -> Vector4 {
        Vector4::new(self / vec.x, self / vec.y, self / vec.z, self / vec.w)
    }
}

impl Neg for Vector4 {
    type Output = Vector4;

    fn neg(self) -> Vector4 {
        Vector4::new(-self.x, -self.y, -self.z, -self.w)
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

mod tests {
    use super::*;

    #[test]
    fn scalar_multiplication_test() {
        let a = Vector4::new(5, 10, 10, 5);
        let res = Vector4::new(10, 20, 20, 10.);

        assert_eq!(a * 2., res);
        assert_eq!(2. * a, res);
    }

    #[test]
    fn scalar_division_test() {
        let vec: Vector4 = Vector4::new(10., 20., 12., 44.);
        assert_eq!(2. / vec, Vector4 { x: 1./5., y: 1./10., z: 1./6., w: 1./22.});
        assert_eq!(vec / 2., Vector4 { x: 5., y: 10., z: 6., w: 22. });
    }

    #[test]
    fn scalar_addition_test() {
        let vec = Vector4::new(10., 20., 6., 123);
        let res = Vector4::new(12., 22., 8., 125.);
        assert_eq!(2. + vec, res);
        assert_eq!(vec + 2., res);
    }

    #[test]
    fn scalar_subtraction_test() {
        let vec: Vector4 = Vector4::new(10., 20., 5., 44.);
        let res: Vector4 = Vector4::new(8, 18, 3, 42.);
        assert_eq!(2. - vec, -res);
        assert_eq!(vec - 2., res);
    }


    #[test]
    fn vector_addition_test() {
        let a: Vector4 = Vector4::new(10., 39., 29., 55.);
        let b: Vector4 = Vector4::new(2., 520., 25., 22.);
        let res: Vector4 = Vector4::new(12, 559, 54, 77.);
        assert_eq!(a + b, res);
        assert_eq!(b + a, res);
    }

    #[test]
    fn vector_subtraction_test() {
        let a: Vector4 = Vector4::new(10., 39., 23., 431.);
        let b: Vector4 = Vector4::new(3., 519., 4., 11.);
        let res: Vector4 = Vector4::new(7, -480, 19, 420);
        assert_eq!(a - b, res);
        assert_eq!(b - a, -res);
    }
    
    #[test]
    fn dot_product_test() {
        let a = Vector4::new(1, 2, 3, 6);
        let b = Vector4::new(3, 4, 5, 643);
        let res = 3884.0;

        assert_eq!(a.dot(b), res);
    }

    #[test]
    fn hadamard_product_test() {
        let a = Vector4::new(10, 2., 12., 321.);
        let b = Vector4::new(4., 2., 9., 91.);
        let res = Vector4::new(40., 4., 108., 29211.);
        assert_eq!(a * b, res);
        assert_eq!(b * a, res);
    }

    #[test]
    fn negation_test() {
        let vec: Vector4 = Vector4::new(10., 20., 5., -12434);
        let res: Vector4 = Vector4::new(-10., -20., -5., 12434);
        assert_eq!(-vec, res);
    }

    #[test]
    fn magnitude_test() {
        let a: Vector4 = Vector4::new(10., 18., 2., 99.);
        assert_eq!(a.length(), f32::sqrt(10229.));
    }

    #[test]
    fn normalize_test() {
        let a: Vector4 = Vector4::new(10, 5, 20, 99);
        let b: Vector4 = Vector4::new(
            10. / f32::sqrt(10326.),
            5. / f32::sqrt(10326.),
            20. / f32::sqrt(10326.),
            99. / f32::sqrt(10326.),
        );

        assert!((a.normalize() - b).length() <= 0.001);
    }

}
