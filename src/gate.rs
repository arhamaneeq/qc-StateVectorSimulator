use crate::matrix::Matrix;

pub struct Gate {
    name: String,
    symbol: String,
    matrix: Matrix,
    target: usize,
    control: Vec<usize>
}

impl Gate {
    pub fn new(_name: &str, _symbol: &str, _matrix: Matrix, _target: usize) -> Gate {
        
        assert!(_matrix.is_unitary());

        Gate {
            matrix: _matrix, 
            name: _name.to_string(), 
            symbol: _symbol.to_string(), 
            target: _target,
            control: vec![]
        }
    }

    pub fn controlled(_name: &str, _symbol: &str, _matrix: Matrix, _target: usize, _controls: Vec<usize>) -> Gate {

        assert!(_matrix.is_unitary());

        Gate {
            matrix: _matrix, 
            name: _name.to_string(), 
            symbol: _symbol.to_string(), 
            target: _target,
            control: _controls,
        }
    }

    pub fn expand(self, n: usize) -> Matrix {
        // TODO
    }
}
