pub trait Scalar: Default + std::ops::Add<Output = Self> + Clone {
    fn one() -> Self;
}

impl Scalar for i32 {
    fn one() -> Self { 1 }
}

impl Scalar for f64 {
    fn one() -> Self { 1.0 }
}

pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::default()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::default(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = vec![vec![T::default(); n]; n];
        for i in 0..n {  // Fixed range
            matrix[i][i] = T::one();
        }
        Matrix(matrix)
    }
}