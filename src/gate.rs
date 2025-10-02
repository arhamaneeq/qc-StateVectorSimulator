use crate::matrix::Matrix;

pub struct Gate {
    matrix: Matrix,
    name: String,
    symbol: String,
    qubits: Vec<usize>,
}

impl Gate {
    pub fn new(_name: &str, _symbol: &str, _matrix: Matrix, _qubits: Vec<usize>) -> Gate {
        assert!(_matrix.is_unitary());
        assert_eq!(1 << _qubits.len(), _matrix.height());

        Gate {
            matrix: _matrix, 
            name: _name.to_string(), 
            symbol: _symbol.to_string(), 
            qubits: _qubits}
        }
}
