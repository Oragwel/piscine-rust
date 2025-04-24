use std::ops::{Mul, Add};
use crate::Matrix;
use crate::Scalar;

impl<T> Mul for Matrix<T>
where
    T: Scalar<Item = T>
        + Mul<Output = T>
        + Add<Output = T>
        + std::iter::Sum<T>
        + Clone,
{
    type Output = Option<Self>;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.number_of_columns() != rhs.number_of_rows() {
            return None;
        }

        let mut result = Matrix::from_elem(self.number_of_rows(), rhs.number_of_columns(), T::zero());

        for j in 0..self.number_of_rows() {
            for i in 0..rhs.number_of_columns() {
                result.0[j][i] = self
                    .row(j)
                    .iter()
                    .cloned()
                    .zip(rhs.col(i).iter().cloned())
                    .map(|(x, y)| x * y)
                    .sum();
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplication() {
        let a = Matrix(vec![vec![1, 2], vec![3, 4]]);
        let b = Matrix(vec![vec![5, 6], vec![7, 8]]);
        let expected = Matrix(vec![vec![19, 22], vec![43, 50]]);
        assert_eq!(a * b, Some(expected));

        let a = Matrix(vec![vec![1, 2]]);
        let b = Matrix(vec![vec![1, 2], vec![3, 4], vec![5, 6]]);
        assert_eq!(a * b, None);
    }
}
