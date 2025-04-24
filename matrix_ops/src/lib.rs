use std::ops::{Add, Sub};

/// A basic Matrix structure wrapping a 2D vector of i32 values.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix(pub Vec<Vec<i32>>);

/// Implement matrix addition.
/// Returns `Some(Matrix)` if dimensions match, otherwise `None`.
impl Add for Matrix {
    type Output = Option<Matrix>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len()
            || self.0.iter().zip(&rhs.0).any(|(a, b)| a.len() != b.len()) {
            return None;
        }

        let result = self.0
            .into_iter()
            .zip(rhs.0)
            .map(|(row_a, row_b)| {
                row_a.into_iter().zip(row_b).map(|(a, b)| a + b).collect()
            })
            .collect();

        Some(Matrix(result))
    }
}

/// Implement matrix subtraction.
/// Returns `Some(Matrix)` if dimensions match, otherwise `None`.
impl Sub for Matrix {
    type Output = Option<Matrix>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len()
            || self.0.iter().zip(&rhs.0).any(|(a, b)| a.len() != b.len()) {
            return None;
        }

        let result = self.0
            .into_iter()
            .zip(rhs.0)
            .map(|(row_a, row_b)| {
                row_a.into_iter().zip(row_b).map(|(a, b)| a - b).collect()
            })
            .collect();

        Some(Matrix(result))
    }
}
