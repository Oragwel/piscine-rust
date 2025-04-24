pub mod ops;
pub mod mult;

use crate::traits::Scalar;

#[derive(Debug, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Matrix<T>
where
    T: Scalar<Item = T> + Clone,
{
    pub fn new() -> Self {
        Matrix(vec![vec![]])
    }

    pub fn from_elem(row: usize, col: usize, value: T) -> Self {
        let mut matrix = vec![];
        for _ in 0..row {
            matrix.push(vec![value.clone(); col]);
        }
        Self(matrix)
    }

    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_columns(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
}
