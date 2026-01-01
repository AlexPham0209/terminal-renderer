use crate::matrix::{matrix3::Matrix3, matrix4::Matrix4};

pub enum Angle {
    Radians(f32),
    Degrees(f32),
}

pub struct Rotation3;
impl Rotation3 {
    pub fn x_rotation_matrix(angle: Angle) -> Matrix3 {
        let angle: f32 = match angle {
            Angle::Degrees(degrees) => degrees.to_radians(),
            Angle::Radians(radians) => radians
        };

        let cos: f32 = f32::cos(angle);
        let sin: f32 = f32::sin(angle);

        Matrix3::new(
            1.0, 0.0, 0.0, 
            0.0, cos, -sin, 
            0.0, sin, cos
        )
    }


    pub fn y_rotation_matrix(angle: Angle) -> Matrix3 {
        let angle: f32 = match angle {
            Angle::Degrees(degrees) => degrees.to_radians(),
            Angle::Radians(radians) => radians
        };

        let cos: f32 = f32::cos(angle);
        let sin: f32 = f32::sin(angle);

        Matrix3::new(
            cos, 0.0, sin,
            0.0, 1.0, 0.0,
            -sin, 0.0, cos
        )
    }

    pub fn z_rotation_matrix(angle: Angle) -> Matrix3 {
        let angle: f32 = match angle {
            Angle::Degrees(degrees) => degrees.to_radians(),
            Angle::Radians(radians) => radians
        };

        let cos: f32 = f32::cos(angle);
        let sin: f32 = f32::sin(angle);

        Matrix3::new(
            cos, - sin, 0.0,
            sin, cos, 0.0,
            0.0, 0.0, 1.0
        )
    }

    pub fn rotation_matrix(yaw: Angle, pitch: Angle, roll: Angle) -> Matrix3 {
        Rotation3::z_rotation_matrix(roll) * 
        Rotation3::y_rotation_matrix(pitch) * 
        Rotation3::x_rotation_matrix(yaw) 
    }
}


pub struct Rotation4;
impl Rotation4 {
    pub fn x_rotation_matrix(angle: Angle) -> Matrix4 {
        Matrix4::from_matrix3(Rotation3::x_rotation_matrix(angle))
    }


    pub fn y_rotation_matrix(angle: Angle) -> Matrix4 {
        Matrix4::from_matrix3(Rotation3::y_rotation_matrix(angle))
    }

    pub fn z_rotation_matrix(angle: Angle) -> Matrix4 {
        Matrix4::from_matrix3(Rotation3::z_rotation_matrix(angle))
    }

    pub fn rotation_matrix(self, yaw: Angle, pitch: Angle, roll: Angle) -> Matrix4 {
        Matrix4::from_matrix3(Rotation3::rotation_matrix(yaw, pitch, roll))
    }
}