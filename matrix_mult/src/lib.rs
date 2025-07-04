use std::ops::{Add, Mul};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Matrix<T>
where
    T: Clone,
{
    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_cols(&self) -> usize {
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

impl<T> Mul for Matrix<T>
where
    T: Clone + Default + Add<Output = T> + Mul<Output = T>,
{
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Self) -> Self::Output {
        let rows_a = self.number_of_rows();
        let cols_a = self.number_of_cols();
        let rows_b = rhs.number_of_rows();
        let cols_b = rhs.number_of_cols();

        if cols_a != rows_b {
            return None;
        }

        let mut result = vec![vec![T::default(); cols_b]; rows_a];

        for i in 0..rows_a {
            for j in 0..cols_b {
                for k in 0..cols_a {
                    result[i][j] = result[i][j].clone()
                        + self.0[i][k].clone() * rhs.0[k][j].clone();
                }
            }
        }

        Some(Matrix(result))
    }
}
