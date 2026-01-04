use crate::matrix::{matrix3::Matrix3, matrix4::Matrix4};

#[derive(Debug, Clone, Copy)]
pub enum Angle {
    Radians(f32),
    Degrees(f32),
}

pub trait Rotation {
    type Output;

    fn x_rotation(angle: Angle) -> Self::Output;
    fn y_rotation(angle: Angle) -> Self::Output;
    fn z_rotation(angle: Angle) -> Self::Output;
    fn rotation(yaw: Angle, pitch: Angle, roll: Angle) -> Self::Output;
}
