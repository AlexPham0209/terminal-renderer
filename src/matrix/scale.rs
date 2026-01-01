pub trait Scale {
    type Output;
    fn scale(scalar: f32) -> Self::Output;
}