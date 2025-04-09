use std::ops::Mul;

#[derive(Clone, Debug)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Matrix<T> {
	pub fn number_of_cols(&self) -> usize {
        self.0[0].len()
	}

	pub fn number_of_rows(&self) -> usize {
        self.0.len()
	}

	pub fn row(&self, n: usize) -> Vec<T> 
    where
        T: Clone,
    {
        self.0[n].clone()
    }
        
	

	pub fn col(&self, n: usize) -> Vec<T> 
    where
        T: Clone,
        
    {
        self.0.iter().map(|row| row[n].clone()).collect()
    }
	
}

impl<T> Mul for Matrix<T> 
where
    T: Clone + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Default,
{
    type Output = Matrix<T>;

    fn mul(self, other: Matrix<T>) -> Matrix<T> {
        let rows_a = self.number_of_rows();
        let cols_a = self.number_of_cols();
        let rows_b = other.number_of_rows();
        let cols_b = other.number_of_cols();

        // Ensure matrix dimensions match for multiplication
        assert_eq!(cols_a, rows_b, "Matrix dimensions do not match for multiplication.");

        let mut result = vec![];

        // Perform the matrix multiplication
        for i in 0..rows_a {
            let mut row = vec![];

            for j in 0..cols_b {
                let mut sum = T::default(); // Default value for T (typically 0 for numbers)
                for k in 0..cols_a {
                    sum = sum + (self.0[i][k].clone() * other.0[k][j].clone());
                }
                row.push(sum);
            }
            result.push(row);
        }

        Matrix(result)
    }
}