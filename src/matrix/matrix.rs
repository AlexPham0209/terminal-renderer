pub trait Matrix {
    type Vector;
    fn row(&self, index: usize) -> Self::Vector;
    fn col(&self, index: usize) -> Self::Vector;
    fn transpose(&self) -> Self;
}
