use lalgebra_scalar::Scalar;

pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    /// Creates a new 1x1 matrix with T::zero()
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::zero()]])
    }

    /// Creates a matrix filled with zeros of size row x col
    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::zero(); col]; row])
    }

    /// Creates an identity matrix of size n x n
    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = vec![vec![T::zero(); n]; n];
        for i in 0..n {
            matrix[i][i] = T::one();
        }
        Matrix(matrix)
    }
}
