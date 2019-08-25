use std::ops::{Index, Mul};
use super::float_cmp;
use super::vec3::Vec3;

#[derive(Debug, Clone)]
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
    pub fn identity_matrix(dim: usize) -> Self {
        let mut matrix = Self::zero_matrix(dim, dim);
        for i in 0..dim {
            matrix.cells[i][i] = 1.0;
        }
        matrix
    }
    pub fn transpose(&self) -> Self {
        Matrix::new(self.columns())
    }
    pub fn determinant(&self) -> f32 {
        assert_eq!(self.n_rows, self.n_cols, "can't computer determinant for non square matrix");
        if (self.n_rows == 2) && (self.n_cols == 2) {
            self.cells[0][0] * self.cells[1][1] - self.cells[0][1] * self.cells[1][0]
        } else {
            let mut det = 0.0;
            for i in 0..self.n_cols {
                det += self[0][i] * self.cofactor(0, i);
            }
            det
        }
    }
    pub fn sub_matrix(&self, row: usize, col: usize) -> Self {
        assert!((row < self.n_rows) && (col < self.n_cols), "submatrix arguments overflow matrix dimensions");
        let mut matrix = Vec::new();
        for i in 0..self.n_rows {
            if i == row { continue; }
            let mut row_vector = Vec::new();
            for j in 0..self.n_cols {
                if j == col { continue; }
                row_vector.push(self.cells[i][j]);
            }
            matrix.push(row_vector);
        }

        Self {
            cells: matrix,
            n_rows: self.n_rows - 1,
            n_cols: self.n_cols - 1,
        }
    }
    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.sub_matrix(row, col).determinant()
    }
    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let sign;
        if (row + col) % 2 == 1 {sign = -1.0;} else {sign = 1.0;}
        sign * self.minor(row, col)
    }
    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }
    pub fn inverse_matrix(&self) -> Self {
        assert!(self.is_invertible(), "matrix is not invertible");
        let mut matrix = Self::zero_matrix(self.n_rows, self.n_cols);
        let det = self.determinant();
        for i in 0..self.n_rows {
            for j in 0..self.n_cols {
                matrix.cells[j][i] = self.cofactor(i, j) / det;
            }
        }
        matrix
    }
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = Self::identity_matrix(4);
        matrix.cells[0][3] = x;
        matrix.cells[1][3] = y;
        matrix.cells[2][3] = z;
        matrix
    }
    pub fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        Self::translation(x, y, z) * self
    }
    pub fn scaling(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = Self::identity_matrix(4);
        matrix.cells[0][0] = x;
        matrix.cells[1][1] = y;
        matrix.cells[2][2] = z;
        matrix
    }
    pub fn scale(&self, x: f32, y: f32, z: f32) -> Self {
        Self::scaling(x, y, z) * self
    }
    pub fn rotation_x(rads: f32) -> Self {
        let mut matrix = Self::identity_matrix(4);
        matrix.cells[1][1] = rads.cos();
        matrix.cells[1][2] = -rads.sin();
        matrix.cells[2][1] = rads.sin();
        matrix.cells[2][2] = rads.cos();
        matrix
    }
    pub fn rotate_x(&self, rads: f32) -> Self {
        Matrix::rotation_x(rads) * self
    }
    pub fn rotation_y(rads: f32) -> Self {
        let mut matrix = Self::identity_matrix(4);
        matrix.cells[0][0] = rads.cos();
        matrix.cells[0][2] = rads.sin();
        matrix.cells[2][0] = -rads.sin();
        matrix.cells[2][2] = rads.cos();
        matrix
    }
    pub fn rotate_y(&self, rads: f32) -> Self {
        Matrix::rotation_y(rads) * self
    }
    pub fn rotation_z(rads: f32) -> Self {
        let mut matrix = Self::identity_matrix(4);
        matrix.cells[0][0] = rads.cos();
        matrix.cells[0][1] = -rads.sin();
        matrix.cells[1][0] = rads.sin();
        matrix.cells[1][1] = rads.cos();
        matrix
    }
    pub fn rotate_z(&self, rads: f32) -> Self {
        Matrix::rotation_z(rads) * self
    }
    pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        let mut matrix = Self::identity_matrix(4);
        matrix.cells[0][1] = xy;
        matrix.cells[0][2] = xz;
        matrix.cells[1][0] = yx;
        matrix.cells[1][2] = yz;
        matrix.cells[2][0] = zx;
        matrix.cells[2][1] = zy;
        matrix
    }
    pub fn shear(&self, xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        Matrix::shearing(xy, xz, yx, yz, zx, zy) * self
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

impl<'a, 'b> Mul<&'b Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &'b Matrix) -> Matrix {
        assert_eq!(self.n_cols, rhs.n_rows, "cannot multiply given matrices");
        let m = self.n_rows;
        let n = rhs.n_cols;
        let mut matrix = Matrix::zero_matrix(m, n);
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

impl<'a> Mul<Vec<f32>> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Vec<f32>) -> Matrix {
        let other_matrix = Matrix::column_matrix(rhs);
        self * &other_matrix
    }
}

impl Mul<Vec3> for Matrix {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let v = (self * rhs.as_vec()).get_tuple();
        Vec3::new(v)
    }
}

impl<'a> Mul<Vec3> for &'a Matrix {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        let v = (self * rhs.as_vec()).get_tuple();
        Vec3::new(v)
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
    #[test]
    fn identity_matrix() {
        let row1 = vec![1.0, 2.0, 3.0, 4.0];
        let row2 = vec![2.0, 4.0, 4.0, 2.0];
        let row3 = vec![8.0, 6.0, 4.0, 1.0];
        let row4 = vec![0.0, 0.0, 0.0, 1.0];
        let matrix = vec![row1, row2, row3, row4];
        let A = Matrix::new( matrix );

        let I = Matrix::identity_matrix(4usize);
        assert_eq!(I * &A, A);
    }
    #[test]
    fn matrix_transpose() {
        let row1 = vec![0.0, 9.0, 3.0, 0.0];
        let row2 = vec![9.0, 8.0, 0.0, 8.0];
        let row3 = vec![1.0, 8.0, 5.0, 3.0];
        let row4 = vec![0.0, 0.0, 5.0, 8.0];
        let matrix = vec![row1, row2, row3, row4];
        let A = Matrix::new( matrix );

        let row1 = vec![0.0, 9.0, 1.0, 0.0];
        let row2 = vec![9.0, 8.0, 8.0, 0.0];
        let row3 = vec![3.0, 0.0, 5.0, 5.0];
        let row4 = vec![0.0, 8.0, 3.0, 8.0];
        let matrix = vec![row1, row2, row3, row4];
        let B = Matrix::new( matrix );

        assert_eq!(A.transpose(), B);
    }
    #[test]
    fn det_2x2() {
        let row1 = vec![1.0, 5.0];
        let row2 = vec![-3.0, 2.0];
        let matrix = vec![row1, row2];
        let A = Matrix::new( matrix );
        assert!(float_cmp::equal(A.determinant(), 17.0));
    }
    #[test]
    fn sub_matrix() {
        let row1 = vec![0.0, 9.0, 3.0, 0.0];
        let row2 = vec![9.0, 8.0, 0.0, 8.0];
        let row3 = vec![1.0, 8.0, 5.0, 3.0];
        let row4 = vec![0.0, 0.0, 5.0, 8.0];
        let matrix = vec![row1, row2, row3, row4];
        let A = Matrix::new( matrix );

        let row1 = vec![0.0, 9.0, 3.0];
        let row2 = vec![9.0, 8.0, 0.0];
        let row3 = vec![1.0, 8.0, 5.0];
        let matrix = vec![row1, row2, row3];
        let B = Matrix::new( matrix );

        assert_eq!(A.sub_matrix(3, 3), B);
    }
    #[test]
    fn minor() {
        let row1 = vec![3.0, 5.0, 0.0];
        let row2 = vec![2.0, -1.0, -7.0];
        let row3 = vec![6.0, -1.0, 5.0];
        let matrix = vec![row1, row2, row3];
        let A = Matrix::new( matrix );

        assert!(float_cmp::equal(A.minor(1, 0), 25.0));
    }
    #[test]
    fn cofactor() {
        let row1 = vec![3.0, 5.0, 0.0];
        let row2 = vec![2.0, -1.0, -7.0];
        let row3 = vec![6.0, -1.0, 5.0];
        let matrix = vec![row1, row2, row3];
        let A = Matrix::new( matrix );

        assert!(float_cmp::equal(A.cofactor(1, 0), -25.0));
    }
    #[test]
    fn determinant_large() {
        let row1 = vec![-2.0, -8.0, 3.0, 5.0];
        let row2 = vec![-3.0, 1.0, 7.0, 3.0];
        let row3 = vec![1.0, 2.0, -9.0, 6.0];
        let row4 = vec![-6.0, 7.0, 7.0, -9.0];
        let matrix = vec![row1, row2, row3, row4];
        let A = Matrix::new( matrix );
        assert!(float_cmp::equal(A.determinant(), -4071.0));
    }
    #[test]
    fn matrix_inverse() {
        let row1 = vec![0.0, 2.0, 3.0, 4.0];
        let row2 = vec![5.0, 6.0, 7.0, 8.0];
        let row3 = vec![9.0, 8.0, 8.0, 6.0];
        let row4 = vec![5.0, 4.0, 3.0, 2.0];
        let matrix = vec![row1, row2, row3, row4];
        let A = Matrix::new( matrix );

        let row1 = vec![-2.0, 1.0, 2.0, 3.0];
        let row2 = vec![3.0, 2.0, 1.0, -1.0];
        let row3 = vec![4.0, 3.0, 6.0, 5.0];
        let row4 = vec![1.0, 2.0, 7.0, 8.0];
        let matrix = vec![row1, row2, row3, row4];
        let B = Matrix::new( matrix );

        let D = A.clone();
        let C = A * &B;

        assert_eq!(C * &B.inverse_matrix(), D);
        let AI = D.inverse_matrix();
        assert_eq!(D * &AI, Matrix::identity_matrix(4));
    }
}
