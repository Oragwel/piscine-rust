use std::fmt::Debug;
use std::ops::{Add, Mul};

pub trait Scalar:
    Add<Output = Self>
    + Mul<Output = Self>
    + Copy
    + Clone
    + Debug
    + PartialEq
    + Eq
{
    fn zero() -> Self;
    fn one() -> Self;
}

impl Scalar for i64 {
    fn zero() -> Self {
        0
    }

    fn one() -> Self {
        1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let data = self.0.iter().zip(other.0.iter()).map(|(a, b)| *a + *b).collect();
        Vector(data)
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let mut result = T::zero();
        for (a, b) in self.0.iter().zip(other.0.iter()) {
            result = result + (*a * *b);
        }
        Some(result)
    }
}
