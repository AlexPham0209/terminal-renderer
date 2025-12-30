use std::{ops::Mul, process::Output};

pub trait Vector {
    type VectorType;

    fn length(&self) -> f32;
    fn normalize(&self) -> Self::VectorType;
    fn dot(&self, other: &Self::VectorType) -> f32;
}
