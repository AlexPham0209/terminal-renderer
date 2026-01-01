pub trait Scale {
    type Output;
    fn scalar(scalar: f32) -> Self::Output;
}