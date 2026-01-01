pub trait Scale {
    type Output;
    fn scalar_matrix(scalar: f32) -> Self::Output;
}