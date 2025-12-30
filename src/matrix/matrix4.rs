use crate::vector::vector4::Vector4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4 {
    x: Vector4,
    y: Vector4,
    z: Vector4,
    w: Vector4,
}
