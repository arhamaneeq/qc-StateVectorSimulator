# Rust Statevector Simulator

This project is a toy quantum statevector simulator written in Rust.
It implements a simple linear algebra backend (complex numbers and matrices) and builds on top of it a small framework for representing and simulating quantum circuits using statevectors.

## Features

### Matrix
- Linear algebra support for complex valued matrices
- Identity, tensor (Kronecker) product, dagger (conjugate) transpose, permutation, and unitary check
- Common gate primitives (X, Y, Z, H, Rx, Ry, Rz) and projectors ($|0\rangle\langle 0|$, $|1\rangle\langle 1|$)

### Gates
- Representation of single and multi-qubit gates within a circuit
- Supports controlled gates (CX, CY, CZ, and Toffoli)
- Gate expansion into the full Hilbert space via tensor products

### Circuit
- Build circuits with any number of qubits
- Add gates
- Compile into unitary operations
- Run to simulate the final statevector

## Example Code
```rust
fn main() {
    let mut qc = Circuit::new(2, 0);

    qc.h(0);
    qc.cx(0, 1);

    qc.compile();
    qc.run().print();
}
```
```shell
Matrix (4 x 1):
-----------------------
|    0.707+   0.000i  |
|    0.000+   0.000i  |
|    0.000+   0.000i  |
|    0.707+   0.000i  |
-----------------------
```

## To-Do

### Basic
- [x] Support basic Complex Arithmetic
- [x] Support basic Matrix Algebra
- [x] Single Qubit Gate Support
- [x] Multi Qubit Gate Support
- [ ] Measurement
### Advanced
- [ ] Parrelisation
- [ ] Simplification
- [ ] Mid-circuit measurements
