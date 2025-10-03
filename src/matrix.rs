use crate::{complex::Complex};

#[derive(Clone, Debug)]
pub struct Matrix {
    matrix: Vec<Complex>,
    width: usize,
    height: usize,
}
#[allow(non_snake_case, dead_code)]
impl Matrix {
    pub fn zeroes(a: [usize; 2]) -> Self {
        let rows: usize = a[0];
        let cols: usize = a[1];

        let matrix: Vec<Complex> = vec![Complex::new(0.0, 0.0); rows * cols];

        Self {
            matrix: matrix,
            width: cols,
            height: rows,
        }
    }

    pub fn print(&self) {
        println!("Matrix ({} x {}):", self.height, self.width);
        println!("----------------------------------");

        for i in 0..self.height {
            print!("| ");
            for j in 0..self.width {
                let v = self[(i, j)];
                print!("{:>8.3}+{:>8.3}i  ", v.real, v.imag);
            }
            println!("|");
        }

        println!("----------------------------------");
    }

    pub fn identity(a: [usize; 2]) -> Self {
        assert_eq!(a[0], a[1], "Matrix should be square");
        let mut matrix: Matrix = Matrix::zeroes(a);

        for i in 0..matrix.width {
            matrix[(i, i)] = Complex::new(1.0, 0.0);
        }

        matrix
    }

    pub fn from_data(data: Vec<Complex>, a: [usize; 2]) -> Matrix {
        assert_eq!(data.len(), a[0] * a[1], "Data length does not match given shape!");

        let w: usize = a[0];
        let h: usize = a[1];

        let mut matrix: Matrix = Matrix::zeroes(a);

        for i in 0..w {
            for j in 0..h {
                matrix[(i,j)] = data[i*w + j];
            }
        }

        matrix
    }

    pub fn I() -> Matrix {
        let matrix: Matrix = Matrix::identity([2, 2]);

        matrix
    }

    pub fn proj0() -> Matrix {
        let matrix: Matrix = Matrix::from_data(
            vec![
                Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)
            ], [2, 2]);

        matrix
    }

    pub fn proj1() -> Matrix {
        let matrix: Matrix = Matrix::from_data(
            vec![
                Complex::new(0.0, 0.0), Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)
            ], [2, 2]);

        matrix
    }

    pub fn X() -> Matrix {
        let matrix: Matrix = Matrix::from_data(
            vec![
                Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), 
                Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)], 
            [2, 2]
        );

        matrix
    }

    pub fn Y() -> Matrix {
        let matrix: Matrix = Matrix::from_data(
            vec![
                Complex::new(0.0, 0.0), Complex::new(0.0, -1.0),
                Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)],
            [2, 2]
        );

        matrix 
    }

    pub fn Z() -> Matrix {
        let matrix: Matrix = Matrix::from_data(
            vec![
                Complex::new(1.0, 0.0), Complex::new(0.0, 0.0),
                Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0)], 
            [2, 2]
        );

        matrix
    }

    pub fn H() -> Matrix {
        let s : f32 = (2.0f32).sqrt().recip();
        let matrix : Matrix = Matrix::from_data(
            vec![
                Complex::new(s, 0.0), Complex::new(s, 0.0),
                Complex::new(s, 0.0), Complex::new(-s, 0.0)
            ], [2,2]);

        matrix
    }

    pub fn Rx(theta: f32) -> Matrix {
        let ct : f32 = (theta / 2.0f32).cos();
        let st : f32 = (theta / 2.0f32).sin();

        let matrix : Matrix = Matrix::from_data(
            vec![
                Complex::new(ct, 0.0), Complex::new(0.0, -st),
                Complex::new(0.0, -st), Complex::new(ct, 0.0)
            ], [2, 2]);

        matrix
    }

    pub fn Ry(theta: f32) -> Matrix {
        let ct : f32 = (theta / 2.0f32).cos();
        let st : f32 = (theta / 2.0f32).sin();

        let matrix : Matrix = Matrix::from_data(
            vec![
                Complex::new(ct, 0.0), Complex::new(- st, 0.0),
                Complex::new(st, 0.0), Complex::new(ct, 0.0)
            ], [2, 2]);

        matrix
    }

    pub fn Rz(theta: f32) -> Matrix {
        let matrix : Matrix = Matrix::from_data(
            vec![
                Complex::polar(1.0, - theta / 2.0), Complex::new(0.0, 0.0),
                Complex::new(0.0, 0.0), Complex::polar(1.0, theta / 2.0)
            ], [2, 2]);

        matrix
    }

    pub fn permutation(n: usize, perm: &[usize]) -> Matrix {
        assert_eq!(perm.len(), n);

        let dim = 1usize << n;
        let mut P = Matrix::zeroes([dim, dim]);

        for i in 0..dim {
            let mut new_index : usize = 0;
            for (new_pos, &old_pos) in perm.iter().enumerate() {
                let bit = (i >> old_pos) & 1;
                new_index |= bit << new_pos;
            }

            P[(new_index, i)] = Complex::new(1.0, 0.0);
        }

        P
    }

    pub fn transpose(&self) -> Self {
        let mut matrix: Matrix = Matrix::zeroes([self.height, self.width]);

        for i in 0..self.height {
            for j in 0..self.width {
                matrix[(i, j)] = self[(j, i)]
            }
        }

        matrix
    }

    pub fn conjugate(&self) -> Self {
        let mut matrix: Matrix = Matrix::zeroes([self.height, self.width]);

        for i in 0..self.height {
            for j in 0..self.width {
                matrix[(i, j)] = self[(i, j)].conj();
            }
        }

        matrix
    }

    pub fn dagger(&self) -> Self {
        self.conjugate().transpose()
    }

    pub fn is_unitary(&self) -> bool {
        self.dagger() * self.clone() == Matrix::identity([self.width, self.height])
    }
}

impl std::cmp::PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.matrix == other.matrix
    }
}

impl std::ops::Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, _rhs: Matrix) -> Matrix {
        assert!(self.width == _rhs.width && self.height == _rhs.height);

        let mut matrix: Matrix = Matrix::zeroes([self.width, self.height]);

        for i in 0..self.width {
            for j in 0..self.height {
                matrix[(i, j)] = self[(i, j)] + _rhs[(i, j)];
            }
        }

        matrix
    }
}

impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, _rhs: Matrix) -> Matrix {
        assert_eq!(self.width, _rhs.height, "Dimension mismatch!");

        let mut result: Matrix = Matrix::zeroes([self.height, _rhs.width]);

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

impl std::ops::BitXor<Matrix> for Matrix {
    // Kronecker Product
    type Output = Matrix;

    fn bitxor(self, _rhs: Matrix) -> Self::Output {
        let m: usize = self.height;
        let n: usize = self.width;
        let p: usize = _rhs.height;
        let q: usize = _rhs.width;

        let mut result: Matrix = Matrix::zeroes([m * p, n * q]);

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