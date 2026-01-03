use crate::{matrix::rotation::Angle, vector::vector3::Vector3};

pub struct Transform {
    pub yaw: Angle,
    pub pitch: Angle,
    pub roll: Angle,
    pub position: Vector3,
    pub scale: f32,
}
