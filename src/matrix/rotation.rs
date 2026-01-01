use crate::matrix::{matrix3::Matrix3, matrix4::Matrix4};

pub enum Angle {
    Radians(f32),
    Degrees(f32),
}

pub trait Rotation {
    type Output;

    fn x_rotation_matrix(angle: Angle) -> Self::Output;
    fn y_rotation_matrix(angle: Angle) -> Self::Output;
    fn z_rotation_matrix(angle: Angle) -> Self::Output;
    fn rotation_matrix(yaw: Angle, pitch: Angle, roll: Angle) -> Self::Output;
}