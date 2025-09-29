use crate::complex::Complex;

#[derive(Clone, Debug)]
pub struct Matrix {
    matrix: Vec<Complex>,
    width: usize,
    height: usize,
}

impl Matrix {
    pub fn new(a: [usize; 2]) -> Self {
        let rows: usize = a[0];
        let cols: usize = a[1];

        let matrix: Vec<Complex> = vec![Complex::new(0.0, 0.0); rows * cols];

        Self {
            matrix: matrix,
            width: cols,
            height: rows,
        }
    }
}

impl std::ops::Mul<&Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self: Matrix, _rhs: &Matrix) -> Matrix {
        assert_eq!(self.width, _rhs.height, "Dimension mismatch!");

        let mut result: Matrix = Matrix::new([self.height, _rhs.width]);

        for i in 0..self.height {
            for j in 0.._rhs.width {
                let mut val: Complex = Complex::new(0.0, 0.0);

                for k in 0..self.width {
                    val = val + self[(i, k)] * _rhs[(k, j)];
                }

                result[(i, j)] = val;
            }
        }

        result
    }
}

impl std::ops::BitXor<&Matrix> for Matrix {
    // Kronecker Product
    type Output = Matrix;

    fn bitxor(self, _rhs: &Matrix) -> Self::Output {
        let m: usize = self.height;
        let n: usize = self.width;
        let p: usize = _rhs.height;
        let q: usize = _rhs.width;

        let mut result: Matrix = Matrix::new([m * p, n * q]);

        for i in 0..m {
            for j in 0..n {
                for k in 0..p {
                    for l in 0..q {
                        result[(i * p + k, j * q + l)] = self[(i, j)] * _rhs[(k, l)];
                    }
                }
            }
        }

        result
    }
}

impl std::ops::Index<(usize, usize)> for Matrix {
    type Output = Complex;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.matrix[i * self.width + j]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[i * self.width + j]
    }
}
