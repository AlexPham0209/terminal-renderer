#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_multiplication_test() {
        assert_eq!(Vector3D {x: 5., y: 10., z: 5.}, Vector3D {x: 5., y: 10., z: 5.});
    }
}