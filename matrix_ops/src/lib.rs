#[derive(Debug, Clone, PartialEq)]
pub struct Matrix(pub Vec<Vec<i32>>);

use std::ops::{Add, Sub};

impl Add for Matrix {
    type Output = Option<Matrix>;

    fn add(self, rhs: Matrix) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0.iter().zip(&rhs.0).any(|(a, b)| a.len() != b.len()) {
            return None;
        }

        let result = self.0.iter().zip(rhs.0.iter())
            .map(|(row1, row2)| {
                row1.iter().zip(row2).map(|(a, b)| a + b).collect::<Vec<i32>>()
            })
            .collect();

        Some(Matrix(result))
    }
}

impl Sub for Matrix {
    type Output = Option<Matrix>;

    fn sub(self, rhs: Matrix) -> Self::Output {
        if self.0.len() != rhs.0.len() || self.0.iter().zip(&rhs.0).any(|(a, b)| a.len() != b.len()) {
            return None;
        }

        let result = self.0.iter().zip(rhs.0.iter())
            .map(|(row1, row2)| {
                row1.iter().zip(row2).map(|(a, b)| a - b).collect::<Vec<i32>>()
            })
            .collect();

        Some(Matrix(result))
    }
}
