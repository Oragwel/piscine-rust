use std::ops::{Add, Sub};
use crate::Matrix;
use crate::Scalar;

impl<T> Add for Matrix<T>
where
    T: Scalar<Item = T> + Add<Output = T> + Clone,
{
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.number_of_rows() != other.number_of_rows()
            || self.number_of_columns() != other.number_of_columns()
        {
            return None;
        }

        let mut matrix = Matrix::new();
        for (j, row) in self.0.iter().enumerate() {
            if j > 0 {
                matrix.0.push(Vec::new());
            }
            for (i, v) in row.iter().enumerate() {
                matrix.0[j].push(v.clone() + other.0[j][i].clone());
            }
        }

        Some(matrix)
    }
}

impl<T> Sub for Matrix<T>
where
    T: Scalar<Item = T> + Sub<Output = T> + Clone,
{
    type Output = Option<Self>;

    fn sub(self, other: Self) -> Self::Output {
        if self.number_of_rows() != other.number_of_rows()
            || self.number_of_columns() != other.number_of_columns()
        {
            return None;
        }

        let mut matrix = Matrix::new();
        for (j, row) in self.0.iter().enumerate() {
            if j > 0 {
                matrix.0.push(Vec::new());
            }
            for (i, v) in row.iter().enumerate() {
                matrix.0[j].push(v.clone() - other.0[j][i].clone());
            }
        }

        Some(matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let expected = Matrix(vec![vec![2, 2], vec![2, 2]]);
        assert_eq!(matrix + matrix_2, Some(expected));

        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
        assert_eq!(matrix + matrix_2, None);
    }

    #[test]
    fn subtraction() {
        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let expected = Matrix(vec![vec![0, 0], vec![0, 0]]);
        assert_eq!(matrix - matrix_2, Some(expected));

        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
        assert_eq!(matrix - matrix_2, None);
    }
}
