use std::ops::{self, Add, Index, Mul, Neg, Sub};

use num::{Num, ToPrimitive, pow};

use crate::vector::vector::Vector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new<T, U>(x: T, y: U) -> Vector2
    where
        T: ToPrimitive,
        U: ToPrimitive,
    {
        Vector2 {
            x: x.to_f32().expect("Not a number"),
            y: y.to_f32().expect("Not a number"),
        }
    }
}

// Vector utilities
impl Vector for Vector2 {
    type VectorType = Vector2;

    fn length(&self) -> f32 {
        f32::sqrt(pow(self.x, 2) + pow(self.y, 2))
    }

    fn normalize(&self) -> Vector2 {
        let length = self.length();
        *self * (1. / length)
    }

    fn dot(&self, other: &Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

// Vector addition
impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        Vector2::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

// Scalar-vector addition
impl Add<f32> for Vector2 {
    type Output = Vector2;
    fn add(self, scalar: f32) -> Vector2 {
        Vector2::new(scalar + self.x, scalar + self.y)
    }
}

impl Add<Vector2> for f32 {
    type Output = Vector2;
    fn add(self, vec: Vector2) -> Vector2 {
        Vector2::new(self + vec.x, self + vec.y)
    }
}

// Scalar-vector subtraction
impl Sub<f32> for Vector2 {
    type Output = Vector2;

    fn sub(self, scalar: f32) -> Vector2 {
        Vector2::new(self.x - scalar, self.y - scalar)
    }
}

impl Sub<Vector2> for f32 {
    type Output = Vector2;
    fn sub(self, vec: Vector2) -> Vector2 {
        Vector2::new(self - vec.x, self - vec.y)
    }
}

// Hadamard product
impl Mul<Vector2> for Vector2 {
    type Output = Vector2;
    fn mul(self, other: Vector2) -> Vector2 {
        Vector2::new(self.x * other.x, self.y * other.y)
    }
}

// Scalar-Vector multiplication
impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, scalar: f32) -> Vector2 {
        Vector2::new(scalar * self.x, scalar * self.y)
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;
    fn mul(self, vec: Vector2) -> Vector2 {
        Vector2::new(self * vec.x, self * vec.y)
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    fn neg(self) -> Vector2 {
        Vector2::new(-self.x, -self.y)
    }
}

// Allows us to index instead of using members
impl Index<usize> for Vector2 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index out of range"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_multiplication_test() {
        let vec: Vector2 = Vector2::new(5., 10.);
        assert_eq!(2. * vec, Vector2 { x: 10., y: 20. });
        assert_eq!(vec * 2., Vector2 { x: 10., y: 20. });
    }

    #[test]
    fn scalar_addition_test() {
        let vec: Vector2 = Vector2::new(10., 20.);
        let res = Vector2::new(12., 22.);
        assert_eq!(2. + vec, res);
        assert_eq!(vec + 2., res);
    }

    #[test]
    fn scalar_subtraction_test() {
        let vec: Vector2 = Vector2::new(10., 20.);
        let res: Vector2 = Vector2::new(8, 18);
        assert_eq!(2. - vec, -res);
        assert_eq!(vec - 2., res);
    }

    #[test]
    fn vector_addition_test() {
        let a: Vector2 = Vector2::new(10., 39.);
        let b: Vector2 = Vector2::new(2., 520.);
        let res: Vector2 = Vector2::new(12, 559);
        assert_eq!(a + b, res);
        assert_eq!(b + a, res);
    }

    #[test]
    fn vector_subtraction_test() {
        let a: Vector2 = Vector2::new(10., 39.);
        let b: Vector2 = Vector2::new(3., 519.);
        let res: Vector2 = Vector2::new(7, -480);
        assert_eq!(a - b, res);
        assert_eq!(b - a, -res);
    }

    #[test]
    fn negation_test() {
        let vec: Vector2 = Vector2::new(10., 20.);
        let res: Vector2 = Vector2::new(-10., -20.);
        assert_eq!(-vec, res);
    }


    #[test]
    fn magnitude_test() {
        let a: Vector2 = Vector2 { x: 3., y: 4. };
        assert_eq!(a.length(), 5.);
    }

    #[test]
    fn dot_product_test() {
        let a: Vector2 = Vector2::new(10., 2.);
        let b: Vector2 = Vector2::new(4., 2.);
        assert_eq!(a.dot(&b), 44.);
        println!("{:?}", b);
    }

    #[test]
    fn hadamard_product_test() {
        let a: Vector2 = Vector2::new(10, 2.);
        let b: Vector2 = Vector2::new(4., 2.);
        let res: Vector2 = Vector2::new(40., 4.);
        assert_eq!(a * b, res);
        assert_eq!(b * a, res);
    }

    #[test]
    fn normalize_test() {
        let a: Vector2 = Vector2::new(3, 4);
        let b: Vector2 = Vector2::new(3./5., 4./5.);
        assert_eq!(a.normalize(), b);
    }

    #[test]
    fn indexing_test() {
        let a = Vector2::new(10, 5);
        assert_eq!(a[0], 10.0);
        assert_eq!(a[1], 5.0);
    }
}
