mod complex;

pub struct Gate {
    let mut matrix : Vec<Vec<Complex>>,
}

impl Gate {
    pub fn new(a: [u, 2]) -> Self {
        Self {
            let rows = a[0];
            let cols = a[1];
            let matrix = vec![vec![0.0; cols]; rows];

            Self { matrix }
        }
    }
}
