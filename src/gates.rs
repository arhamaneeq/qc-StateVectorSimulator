use crate::complex::Complex;

pub struct Gate {
    matrix: Vec<Vec<Complex>>,
}

impl Gate {
    pub fn new(a: [usize; 2]) -> Self {
        let rows: usize = a[0];
        let cols: usize = a[1];

        let matrix: Vec<Vec<Complex>> = vec![vec![Complex::new(0.0, 0.0); cols]; rows];

        Self { matrix }
    }
}
