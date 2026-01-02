use std::ops::{Add, Div, Index, Mul, Neg, Sub};

use approx::{AbsDiffEq, abs_diff_eq};

use crate::{
    matrix::{
        matrix::Matrix,
        matrix3::Matrix3,
        rotation::{Angle, Rotation},
        scale::Scale,
    },
    vector::{vector3::Vector3, vector4::Vector4},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4 {
    pub x: Vector4,
    pub y: Vector4,
    pub z: Vector4,
    pub w: Vector4,
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

    pub fn from_rows(x: Vector4, y: Vector4, z: Vector4, w: Vector4) -> Self {
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

    pub fn to_homogenous(mat: Matrix3) -> Matrix4 {
        let x = Vector4::to_vector4(mat.x, 0.0);
        let y = Vector4::to_vector4(mat.y, 0.0);
        let z = Vector4::to_vector4(mat.z, 0.0);
        let w = Vector4::new(0.0, 0.0, 0.0, 1.0);
        Matrix4::from_cols(x, y, z, w)
    }

    pub fn cartesian(&self) -> Matrix3 {
        Matrix3::to_cartesian(*self)
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

    fn identity() -> Self {
        let x = Vector4::new(1, 0, 0, 0);
        let y = Vector4::new(0, 1, 0, 0);
        let z = Vector4::new(0, 0, 1, 0);
        let w = Vector4::new(0, 0, 0, 1);
        Matrix4::from_cols(x, y, z, w)
    }
}

impl Matrix4 {
    pub fn translation(t: Vector3) -> Matrix4 {
        let x = Vector4::new(1, 0, 0, 0);
        let y = Vector4::new(0, 1, 0, 0);
        let z = Vector4::new(0, 0, 1, 0);
        let w = Vector4::to_homogeneous(t);
        Matrix4::from_cols(x, y, z, w)
    }

    pub fn view(yaw: Angle, pitch: Angle, roll: Angle, t: Vector3) -> Matrix4 {
        Matrix4::rotation(yaw, pitch, roll).transpose() * Matrix4::translation(-t)
    }

    pub fn perspective(fov: Angle, z_far: f32, z_near: f32, aspect: f32) -> Matrix4 {
        let fov: f32 = match fov {
            Angle::Degrees(degrees) => degrees.to_radians(),
            Angle::Radians(radians) => radians,
        };

        let tan = f32::tan(fov / 2.0);

        let x = Vector4::new(1.0 / (aspect * tan), 0, 0, 0);
        let y = Vector4::new(0, 1.0 / tan, 0, 0);
        let z = Vector4::new(0, 0, -((z_far + z_near) / (z_far - z_near)), -1);
        let w = Vector4::new(0, 0, -((2.0 * z_far * z_near) / (z_far - z_near)), 0);

        // Z coordinate is scaled so that z values between z_near and z_far are normalized to (0, 1) range.
        // If value has a z_value greater than 1 or less than 0, then it is outside the view frustrum and we don't render.
        Matrix4::from_cols(x, y, z, w)
    }

    pub fn orthographic(t: f32, b: f32, r: f32, l: f32, f: f32, n: f32) -> Matrix4 {
        let x = Vector4::new(2.0 / (r - l), 0, 0, 0);
        let y = Vector4::new(0, 2.0 / (t - b), 0, 0);
        let z = Vector4::new(0, 0, -2.0 / (f - n), 0);
        let w = Vector4::new((l + r) / 2.0, (t + b) / 2.0, -(f + n) / 2.0, 1);
        Matrix4::from_cols(x, y, z, w)
    }
}

impl Scale for Matrix4 {
    type Output = Matrix4;

    fn scale(scalar: f32) -> Matrix4 {
        (scalar * Matrix3::identity()).homogenous()
    }
}

// For rotation matrices
impl Rotation for Matrix4 {
    type Output = Matrix4;
    fn x_rotation(angle: Angle) -> Matrix4 {
        Matrix4::to_homogenous(Matrix3::x_rotation(angle))
    }

    fn y_rotation(angle: Angle) -> Matrix4 {
        Matrix4::to_homogenous(Matrix3::y_rotation(angle))
    }

    fn z_rotation(angle: Angle) -> Matrix4 {
        Matrix4::to_homogenous(Matrix3::z_rotation(angle))
    }

    fn rotation(yaw: Angle, pitch: Angle, roll: Angle) -> Matrix4 {
        Matrix4::to_homogenous(Matrix3::rotation(yaw, pitch, roll))
    }
}

// Matrix-Vector Multiplication
impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;

    fn mul(self, v: Vector4) -> Self::Output {
        v.x * self.x + v.y * self.y + v.z * self.z + v.w * self.w
    }
}

impl Mul<Vector3> for Matrix4 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Self::Output {
        let v = v.homogenous();
        let v = v.x * self.x + v.y * self.y + v.z * self.z + v.w * self.w;
        v.cartesian()
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

impl Mul<Matrix4> for f32 {
    type Output = Matrix4;

    fn mul(self, mat: Matrix4) -> Self::Output {
        let x = self * mat.x;
        let y = self * mat.y;
        let z = self * mat.z;
        let w = self * mat.w;
        Matrix4::from_cols(x, y, z, w)
    }
}

// Scalar-Matrix multiplication
impl Div<f32> for Matrix4 {
    type Output = Matrix4;

    fn div(self, scalar: f32) -> Self::Output {
        let x = self.x / scalar;
        let y = self.y / scalar;
        let z = self.z / scalar;
        let w = self.w / scalar;
        Matrix4::from_cols(x, y, z, w)
    }
}

impl Div<Matrix4> for f32 {
    type Output = Matrix4;

    fn div(self, mat: Matrix4) -> Self::Output {
        let x = self / mat.x;
        let y = self / mat.y;
        let z = self / mat.z;
        let w = self / mat.w;
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

impl Add<Matrix4> for f32 {
    type Output = Matrix4;

    fn add(self, mat: Matrix4) -> Self::Output {
        let x = mat.x + self;
        let y = mat.y + self;
        let z = mat.z + self;
        let w = mat.w + self;
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

// Matrix negation
impl Neg for Matrix4 {
    type Output = Matrix4;

    fn neg(self) -> Matrix4 {
        Matrix4::from_cols(-self.x, -self.y, -self.z, -self.w)
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

// For approximate equals
impl AbsDiffEq for Matrix4
where
    <f32 as AbsDiffEq>::Epsilon: Copy,
{
    type Epsilon = <f32 as AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        Self::Epsilon::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        // println!("{}", abs_diff_eq!(&self.x, &other.x, epsilon=epsilon));
        // println!("{}", abs_diff_eq!(&self.y, &other.y, epsilon=epsilon));
        // println!("{}", abs_diff_eq!(&self.z, &other.z, epsilon=epsilon));
        abs_diff_eq!(&self.x, &other.x, epsilon = epsilon)
            && abs_diff_eq!(&self.y, &other.y, epsilon = epsilon)
            && abs_diff_eq!(&self.z, &other.z, epsilon = epsilon)
            && abs_diff_eq!(&self.w, &other.w, epsilon = epsilon)
    }

    fn abs_diff_ne(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        !Self::abs_diff_eq(self, other, epsilon)
    }
}

mod tests {
    use core::f32;

    use approx::assert_abs_diff_eq;

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
        let a = Matrix4::from_rows(
            Vector4::new(10, 2, 3, 12),
            Vector4::new(5, 12, 11, 5),
            Vector4::new(9, 1, 4, 912),
            Vector4::new(5, 23, 11, 55),
        );
        let t = Matrix4::from_rows(
            Vector4::new(10, 5, 9, 5),
            Vector4::new(2, 12, 1, 23),
            Vector4::new(3, 11, 4, 11),
            Vector4::new(12, 5, 912, 55),
        );

        assert_eq!(a.transpose(), t)
    }

    #[test]
    fn from_matrix3_test() {
        let a = Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);

        let res = Matrix4::new(
            1.0, 2.0, 3.0, 0.0, 4.0, 5.0, 6.0, 0.0, 7.0, 8.0, 9.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );

        assert_eq!(Matrix4::to_homogenous(a), res)
    }

    #[test]
    fn translation_matrix_test() {
        let t: Vector3 = Vector3::new(1, 3, 5);
        let res: Matrix4 = Matrix4::new(
            1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 3.0, 0.0, 0.0, 1.0, 5.0, 0.0, 0.0, 0.0, 1.0,
        );
        assert_abs_diff_eq!(Matrix4::translation(t), res)
    }

    #[test]
    fn matrix_vector_multiplication_test() {
        let a = Matrix4::new(
            1.0, 2.0, 3.0, 9.1, 2.1, 12.0, 29.0, 55.0, 11.1, 3.0, 123.5, 12.0, 43.1, 31.1, 5.1, 1.0,
        );
        let b = Vector4::new(2, 3, 4, 5);
        let res = Vector4::new(65.5, 431.2, 585.2, 204.9);
        assert_eq!(a * b, res);
    }

    #[test]
    fn matrix_multiplication_test() {
        let a = Matrix4::new(
            1.0, 2.0, 3.0, 9.1, 2.1, 12.0, 29.0, 55.0, 11.1, 3.0, 123.5, 12.0, 43.1, 31.1, 5.1, 1.0,
        );
        let b = Matrix4::new(
            5.0, 29.0, 7.0, 6.3, 4.4, 39.0, 55.0, 125.0, 9.7, 3.0, 5.2, 12.0, 43.1, 31.1, 7.1, 1.0,
        );
        let res = Matrix4::new(
            435.11, 399.01, 197.21, 301.4, 2715.1, 2326.4, 1216.0, 1916.23, 1783.85, 1182.6, 970.1,
            1938.93, 444.91, 2509.2, 2045.82, 4221.23,
        );
        assert_abs_diff_eq!(a * b, res, epsilon = 1e-3);
    }

    #[test]
    fn matrix_scalar_multiplication_test() {
        let a = Matrix4::new(
            1.0, 2.0, 3.0, 9.1, 2.1, 12.0, 29.0, 55.0, 11.1, 3.0, 123.5, 12.0, 43.1, 31.1, 5.1, 1.0,
        );
        let res = Matrix4::new(
            2.0, 2.0, 3.0, 9.1, 4.2, 12.0, 29.0, 55.0, 22.2, 3.0, 123.5, 12.0, 43.1, 31.1, 5.1, 1.0,
        );
        assert_abs_diff_eq!(a * 2., res);
        assert_abs_diff_eq!(2. * a, res);
    }

    // #[test]
    // fn matrix_scalar_division_test() {
    //     let a = Matrix2::new(2.0, 4.0, 6.0, 4.0);
    //     let b = Matrix2::new(1.0, 2.0, 3.0, 2.0);
    //     let c = Matrix2::new(1.0, 0.5, 0.33333334, 0.5);
    //     assert_eq!(a / 2., b);
    //     assert_eq!(2. / a, c);
    // }

    // #[test]
    // fn matrix_scalar_addition_test() {
    //     let a = Matrix3::new(
    //         1.0, 2.0, 3.0,
    //         2.1, 12.0, 29.0,
    //         11.1, 3.0, 123.5,
    //     );
    //     let res = Matrix3::new(
    //         3.0, 4.0, 5.0,
    //         4.1, 14.0, 31.0,
    //         13.1, 5.0, 125.5,
    //     );
    //     assert_eq!(a + 2., res);
    //     assert_eq!(2. + a, res);
    // }

    // #[test]
    // fn matrix_scalar_subtraction_test() {
    //     let a = Matrix3::new(
    //         1.0, 2.0, 3.0,
    //         5.0, 12.0, 29.0,
    //         11.1, 3.0, 123.5,
    //     );
    //     let res = Matrix3::new(
    //         -1.0, 0.0, 1.0,
    //         3.0, 10.0, 27.0,
    //         9.1, 1.0, 121.5,
    //     );
    //     assert_eq!(a - 2., res);
    //     assert_eq!(2. - a, -res);
    // }

    // #[test]
    // fn matrix_addition_test() {
    //     let a = Matrix3::new(
    //         1.0, 2.0, 3.0,
    //         2.11, 12.0, 29.0,
    //         11.1, 3.0, 123.5,
    //     );
    //     let b = Matrix3::new(
    //         5.5, 1.1, 6.0,
    //         9.5, 111.0, 74.0,
    //         81.1, 99.0, -2.0,
    //     );
    //     let res = Matrix3::new(
    //         6.5, 3.1, 9.0,
    //         11.61, 123.0, 103.0,
    //         92.2, 102.0, 121.5,
    //     );
    //     assert_eq!(a + b, res);
    //     assert_eq!(b + a, res);
    // }

    // #[test]
    // fn matrix_subtraction_test() {
    //     let a = Matrix3::new(
    //         1.0, 2.0, 3.0,
    //         2.1, 12.0, 29.0,
    //         11.1, 3.0, 123.5,
    //     );
    //     let b = Matrix3::new(
    //         5.5, 1.1, 6.0,
    //         9.5, 111.0, 74.0,
    //         81.1, 99.0, -2.0,
    //     );
    //     let res = Matrix3::new(
    //         -4.5, 0.9, -3.0,
    //         -7.4, -99.0, -45.0,
    //         -70.0, -96.0, 125.5,
    //     );
    //     assert_eq!(a - b, res);
    //     assert_eq!(b - a, -res);
    // }
}
