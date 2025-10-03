use crate::matrix::Matrix;

#[allow(dead_code)]
pub struct Gate {
    name: String,
    symbol: String,
    matrix: Matrix,
    target: usize,
    control: Vec<usize>
}

impl Gate {
    pub fn new(_name: &str, _symbol: &str, _matrix: Matrix, _target: usize) -> Gate {
        
        // assert!(_matrix.is_unitary());

        Gate {
            matrix: _matrix, 
            name: _name.to_string(), 
            symbol: _symbol.to_string(), 
            target: _target,
            control: vec![]
        }
    }

    pub fn controlled(_name: &str, _symbol: &str, _matrix: Matrix, _target: usize, _controls: Vec<usize>) -> Gate {

        // assert!(_matrix.is_unitary());

        Gate {
            matrix: _matrix, 
            name: _name.to_string(), 
            symbol: _symbol.to_string(), 
            target: _target,
            control: _controls,
        }
    }

    pub fn expand(&self, n: usize) -> Matrix {
        let mut unitary : Matrix = Matrix::identity([1,1]);

        if self.is_controlled() {
            let mut p0_c : Matrix = Matrix::proj0();
            let mut p1_c : Matrix = Matrix::proj1();

            for _ in 1..self.control.len() {
                p0_c = p0_c ^ Matrix::proj0();
                p1_c = p1_c ^ Matrix::proj1();
            }

            let base : Matrix = (p0_c ^ Matrix::I()) + (p1_c ^ self.matrix.clone());
            let trailing = n - self.control.len() - 1;
            let mut embed = base.clone();
            for _ in 0..trailing {
                embed = embed ^ Matrix::I();
            }

            let mut perm = self.control.clone();
            perm.push(self.target);

            let mut remaining: Vec<usize> = (0..n).filter(|q| !perm.contains(q)).collect();
            perm.append(&mut remaining);

            let p: Matrix = Matrix::permutation(n, &perm);

            unitary = p.dagger() * embed * p;
        } else {
            for i in 0..n {
                if i == self.target {
                    unitary = unitary ^ self.matrix.clone();
                } else {
                    unitary = unitary ^ Matrix::I();
                }
            }
        }

        unitary
    }

    pub fn is_controlled(&self) -> bool {
        self.control.len() != 0
    }
}
