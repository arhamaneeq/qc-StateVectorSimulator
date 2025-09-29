use crate::matrix::Matrix;

struct Gate {
    matrix: Matrix,
    name: String,
    qubits: Vec<usize>,
}
