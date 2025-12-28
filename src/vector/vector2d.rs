use std::{ops};

use crate::vector::vector::Vector;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D { x, y }
    }
}

// Vector utilities
impl Vector for Vector2D {
    type VectorType = Vector2D;
    
    fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }

    fn normalize(&self) -> Vector2D {
        let length = self.length();
        *self * (1./length)
    }

    fn dot(&self, other: &Vector2D) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

// Vector addition
impl ops::Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Scalar-vector addition
impl ops::Add<f32> for Vector2D {
    type Output = Vector2D;
    fn add(self, scalar: f32) -> Vector2D {
        Vector2D::new(scalar + self.x, scalar + self.y)
    }
}

impl ops::Add<Vector2D> for f32 {
    type Output = Vector2D;
    fn add(self, vec: Vector2D) -> Vector2D {
        Vector2D::new(self + vec.x, self + vec.y)
    }
}

// Hadamard product
impl ops::Mul<Vector2D> for Vector2D {
    type Output = Vector2D;
    fn mul(self, other: Vector2D) -> Vector2D {
        Vector2D::new(self.x * other.x, self.y * other.y)
    }
}

// Scalar-Vector multiplication
impl ops::Mul<f32> for Vector2D {
    type Output = Vector2D;
    fn mul(self, scalar: f32) -> Vector2D {
        Vector2D::new(scalar * self.x, scalar * self.y)
    }
}

impl ops::Mul<Vector2D> for f32 {
    type Output = Vector2D;
    fn mul(self, vec: Vector2D) -> Vector2D {
        Vector2D::new(self * vec.x, self * vec.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_multiplication_test() {
        let vec: Vector2D = Vector2D::new(5., 10.);
        assert_eq!(2. * vec, Vector2D { x: 10., y: 20. });
        assert_eq!(vec * 2., Vector2D { x: 10., y: 20. });
    }

    #[test]
    fn scalar_addition_test() {
        let vec: Vector2D = Vector2D::new(10., 20.);
        assert_eq!(2. + vec, Vector2D { x: 12., y: 22. });
        assert_eq!(vec + 2., Vector2D { x: 12., y: 22. });
    }
    
    #[test]
    fn magnitude_test() {
        let a: Vector2D = Vector2D { x: 3., y: 4. };
        assert_eq!(a.length(), 5.);
    }

    #[test]
    fn dot_product_test() {
        let a: Vector2D = Vector2D::new(10., 2.);
        let b: Vector2D = Vector2D::new(4., 2.);
        assert_eq!(a.dot(&b), 44.);
        println!("{:?}", b);
    }

    #[test]
    fn hadamard_product_test() {
        let a: Vector2D = Vector2D::new(10., 2.);
        let b: Vector2D = Vector2D::new(4., 2.);
        let res: Vector2D = Vector2D::new(40., 4.);
        assert_eq!(a * b, res);
        assert_eq!(b * a, res);
    }


    #[test]
    fn normalize_test() {
        let a: Vector2D = Vector2D { x: 3., y: 4. };
        let b: Vector2D = Vector2D { x: 3./5., y: 4./5. };
        assert_eq!(a.normalize(), b);
    }
    

}