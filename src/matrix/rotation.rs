use crate::matrix::matrix3::Matrix3;

pub enum Angle {
    Radians(f32),
    Degrees(f32),
}

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
    z_rotation_matrix(roll) * y_rotation_matrix(pitch) * x_rotation_matrix(yaw) 
}
