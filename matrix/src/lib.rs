// matrix/src/lib.rs

pub struct Matrix<T>(Vec<Vec<T>>);

impl<T: Scalar<Item = T> + Clone> Matrix<T> {
    // Constructor to create a new Matrix from elements
    pub fn from_elem(row: usize, col: usize, value: T) -> Self {
        let mut matrix = Vec::with_capacity(row);
        for _ in 0..row {
            matrix.push(vec![value.clone(); col]);
        }
        Self(matrix)
    }

    // `zero()` function to create a zero matrix of size `row x col`
    pub fn zero(row: usize, col: usize) -> Self {
        Self::from_elem(row, col, T::zero())
    }

    // `identity()` function to create an identity matrix of size `n x n`
    pub fn identity(n: usize) -> Self {
        let mut matrix = Self::zero(n, n);
        for i in 0..n {
            matrix.0[i][i] = T::one();
        }
        matrix
    }

    // Additional functionality like matrix multiplication can be added here...
}

pub trait Scalar: Copy + Clone + std::ops::Add<Output = Self> + std::ops::Mul<Output = Self> + Default {
    fn zero() -> Self;
    fn one() -> Self;
}

// Implement Scalar for i32
impl Scalar for i32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}

// Implement Scalar for f64
impl Scalar for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
}

// Implement Scalar for u32
impl Scalar for u32 {
    fn zero() -> Self { 0 }
    fn one() -> Self { 1 }
}
