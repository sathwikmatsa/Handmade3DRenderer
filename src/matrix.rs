use std::ops::{Index, Mul};
use super::float_cmp;

#[derive(Debug)]
pub struct Matrix {
    pub cells: Vec<Vec<f32>>,
    pub n_rows: usize,
    pub n_cols: usize,
}

impl Matrix {
    pub fn new(matrix: Vec<Vec<f32>>) -> Self {
        let n = matrix[0].len();
        let m = matrix.len();
        for i in 1..m {
            assert_eq!(n, matrix[i].len(), "Not a rectangular array");
        }
        Self {
            cells: matrix,
            n_rows: m,
            n_cols: n,
        }
    }
    pub fn row_matrix(vector: Vec<f32>) -> Self {
        let n = vector.len();
        let mut matrix = Vec::new();
        matrix.push(vector);
        Self {
            cells: matrix,
            n_rows: 1,
            n_cols: n,
        }
    }
    pub fn column_matrix(vector: Vec<f32>) -> Self {
        let m = vector.len();
        let mut matrix = Vec::new();
        for ele in vector.iter() {
            let mut vec = Vec::new();
            vec.push(*ele);
            matrix.push(vec);
        }
        Self {
            cells: matrix,
            n_rows: m,
            n_cols: 1,
        }
    }
    pub fn get_tuple(self) -> Vec<f32> {
        assert_eq!(self.n_cols, 1);
        let mut vector = Vec::new();
        for row in self.cells.iter() {
            vector.push(row[0]);
        }
        vector
    }
    pub fn columns(&self) -> Vec<Vec<f32>> {
        let mut columns = Vec::new();
        for j in 0..self.n_cols {
            let mut column_vector = Vec::new();
            for i in 0..self.n_rows {
                column_vector.push(self.cells[i][j]);
            }
            columns.push(column_vector);
        }
        columns
    }
    pub fn zero_matrix(rows: usize, cols: usize) -> Self {
        let mut row_vector = Vec::new();
        for _ in 0..cols {
            row_vector.push(0.0);
        }
        let mut matrix = Vec::new();
        for _ in 0..rows {
            matrix.push(row_vector.clone());
        }
        Self {
            cells: matrix,
            n_rows: rows,
            n_cols: cols,
        }
    }
}

impl Index<usize> for Matrix {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl Mul<&Self> for Matrix {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self {
        assert_eq!(self.n_cols, rhs.n_rows, "cannot multiply given matrices");
        let m = self.n_rows;
        let n = rhs.n_cols;
        let mut matrix = Self::zero_matrix(m, n);
        for (i, row) in self.cells.iter().enumerate() {
            for (j, col) in rhs.columns().iter().enumerate() {
                let mut dot_product = 0.0;
                for k in 0..row.len() {
                    dot_product += row[k] * col[k];
                }
                matrix.cells[i][j] = dot_product;
            }
        }
        matrix
    }
}

impl Mul<Vec<f32>> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Vec<f32>) -> Self {
        let other_matrix = Self::column_matrix(rhs);
        self * &other_matrix
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if (self.n_rows == other.n_rows) && (self.n_cols == other.n_cols) {
            for i in 0..self.n_rows {
                for j in 0..self.n_cols {
                    if !float_cmp::equal(self.cells[i][j], other.cells[i][j]) {
                        return false;
                    }
                }
            }
            return true;
        }
        false
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn create_matrix() {
        let row1 = vec![1.0, 2.0, 3.0, 4.0];
        let row2 = vec![-1.0, -2.0, -3.0, -4.0];
        let row3 = vec![0.0, 0.0, 0.0, 0.0];
        let row4 = vec![1.0, 1.0, -1.0, -1.0];
        let matrix = vec![row1, row2, row3, row4];
        let i = Matrix::new( matrix );
        assert_eq!(i.cells[0], vec![1.0, 2.0, 3.0, 4.0]);
    }
    #[test]
    fn indexing_matrix() {
        let row1 = vec![-3.0, 5.0, 0.0];
        let row2 = vec![1.0, -2.0, -7.0];
        let row3 = vec![0.0, 1.0, 1.0];
        let m = vec![row1, row2, row3];
        let matrix = Matrix::new(m);
        assert_eq!(matrix[1][2], -7.0);
    }
    #[test]
    fn matrix_equality() {
        let row1 = vec![-3.0, 5.0, 0.0];
        let row2 = vec![1.0, -2.0, -7.0];
        let row3 = vec![0.0, 1.0, 1.0];
        let m1 = vec![row1, row2, row3];
        let matrix1 = Matrix::new(m1);

        let row1 = vec![-3.0, 5.0, 0.0];
        let row2 = vec![1.0, -2.0, -7.0];
        let row3 = vec![0.0, 1.0, 1.0];
        let m2 = vec![row1, row2, row3];
        let matrix2 = Matrix::new(m2);

        let row1 = vec![-3.0, 5.0, 0.0];
        let row2 = vec![1.0, -2.0, -7.0];
        let row3 = vec![0.0, 1.0, 2.0];
        let m3 = vec![row1, row2, row3];
        let matrix3 = Matrix::new(m3);

        assert!(matrix1 == matrix2);
        assert!(matrix1 != matrix3);
    }
    #[test]
    fn matrix_multiplication() {

        let row1 = vec![1.0, 2.0, 3.0, 4.0];
        let row2 = vec![5.0, 6.0, 7.0, 8.0];
        let row3 = vec![9.0, 8.0, 7.0, 6.0];
        let row4 = vec![5.0, 4.0, 3.0, 2.0];
        let matrix = vec![row1, row2, row3, row4];
        let A = Matrix::new( matrix );

        let row1 = vec![-2.0, 1.0, 2.0, 3.0];
        let row2 = vec![3.0, 2.0, 1.0, -1.0];
        let row3 = vec![4.0, 3.0, 6.0, 5.0];
        let row4 = vec![1.0, 2.0, 7.0, 8.0];
        let matrix = vec![row1, row2, row3, row4];
        let B = Matrix::new( matrix );

        let row1 = vec![20.0, 22.0, 50.0, 48.0];
        let row2 = vec![44.0, 54.0, 114.0, 108.0];
        let row3 = vec![40.0, 58.0, 110.0, 102.0];
        let row4 = vec![16.0, 26.0, 46.0, 42.0];
        let matrix = vec![row1, row2, row3, row4];
        let C = Matrix::new( matrix );

        assert_eq!(C, A * &B);

    }

    #[test]
    fn multiply_with_vector() {
        let row1 = vec![1.0, 2.0, 3.0, 4.0];
        let row2 = vec![2.0, 4.0, 4.0, 2.0];
        let row3 = vec![8.0, 6.0, 4.0, 1.0];
        let row4 = vec![0.0, 0.0, 0.0, 1.0];
        let matrix = vec![row1, row2, row3, row4];
        let A = Matrix::new( matrix );

        assert_eq!((A * vec![1.0, 2.0, 3.0, 1.0]).get_tuple(), vec![18.0, 24.0, 33.0, 1.0]);
    }
}
