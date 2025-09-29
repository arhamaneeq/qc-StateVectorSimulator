use crate::complex::Complex;

pub struct Matrix {
    matrix: Vec<Vec<Complex>>,
    width: usize,
    height: usize,
}

impl Matrix {
    pub fn new(a: [usize; 2]) -> Self {
        let rows: usize = a[0];
        let cols: usize = a[1];

        let matrix: Vec<Vec<Complex>> = vec![vec![Complex::new(0.0, 0.0); cols]; rows];

        Self {
            matrix: matrix,
            width: cols,
            height: rows,
        }
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, _rhs: Matrix) -> Matrix {
        // TODO: create accessors for matrix
    }
}

impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = Complex;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.matrix[i][j]
    }
}
