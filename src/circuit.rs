use crate::{complex::Complex, gate::Gate, matrix::Matrix};

#[allow(dead_code)]
pub struct Circuit {
    qbits: usize,
    cbits: usize,
    gates: Vec<Gate>,
    unitaries: Vec<Matrix>
}

#[allow(dead_code, non_snake_case)]
impl Circuit {
    pub fn new(_qbits: usize, _cbits: usize) -> Self {
        Self {
            qbits: _qbits,
            cbits: _cbits,
            gates: Vec::new(),
            unitaries: Vec::new(),
        }
    }

    pub fn compile(&mut self) {
        let n: usize = self.qbits;

        for gate in &self.gates {
            self.unitaries.push(gate.expand(n));
        }
    }

    pub fn run(&self) -> Matrix {
        let n: usize = self.qbits;

        let mut statevector: Matrix = Matrix::zeroes([1 << n, 1]);
        statevector[(0, 0)] = Complex::new(1.0, 0.0);

        for unitary in &self.unitaries {
            statevector = unitary.clone() * statevector;
        }

        statevector
    }

    pub fn iter(&self) -> std::ops::Range<usize> {
        0..self.qbits
    }

    pub fn h(&mut self, qbit: usize) {
        let h_gate: Gate = Gate::new(
            "Hadamard",
            "H", 
            Matrix::H(), 
            qbit % self.qbits,
        );

        self.gates.push(h_gate);
    }

    pub fn x(&mut self, qbit: usize) {
        let x_gate: Gate = Gate::new(
            "Pauli X",
            "X",
            Matrix::X(),
            qbit % self.qbits,
        );

        self.gates.push(x_gate);
    }

    pub fn y(&mut self, qbit: usize) {
        let y_gate: Gate = Gate::new(
            "Pauli Y",
            "Y",
            Matrix::Y(),
            qbit % self.qbits
        );

        self.gates.push(y_gate);
    }

    pub fn z(&mut self, qbit: usize) {
        let z_gate: Gate = Gate::new(
            "Pauli Z",
            "Z",
            Matrix::Z(),
            qbit % self.qbits,
        );

        self.gates.push(z_gate);
    }

    pub fn rx(&mut self, theta: f32, qbit: usize) {
        let r_gate: Gate = Gate::new(
            "Rotation X",
            "Rx",
            Matrix::Rx(theta),
            qbit % self.qbits
        );

        self.gates.push(r_gate);
    }

    pub fn ry(&mut self, theta: f32, qbit: usize) {
        let r_gate: Gate = Gate::new(
            "Rotation Y",
            "Ry",
            Matrix::Ry(theta),
            qbit % self.qbits
        );

        self.gates.push(r_gate);
    }

    pub fn rz(&mut self, theta: f32, qbit: usize) {
        let r_gate: Gate = Gate::new(
            "Rotation Z",
            "Rz",
            Matrix::Rz(theta),
            qbit % self.qbits
        );

        self.gates.push(r_gate);
    }

    pub fn cx(&mut self, control: usize, target: usize) {
        assert_ne!(control, target);

        let c_gate : Gate = Gate::controlled(
            "Controlled X",
            "CX",
            Matrix::X(),
            target % self.qbits,
            vec![control % self.qbits]

        );

        self.gates.push(c_gate);
    }

    pub fn cy(&mut self, control: usize, target: usize) {
        assert_ne!(control, target);

        let c_gate : Gate = Gate::controlled(
            "Controlled Y",
            "CY",
            Matrix::Y(),
            target % self.qbits,
            vec![control % self.qbits]

        );

        self.gates.push(c_gate);
    }

    pub fn cz(&mut self, control: usize, target: usize) {
        assert_ne!(control, target);

        let c_gate : Gate = Gate::controlled(
            "Controlled Z",
            "CZ",
            Matrix::Z(),
            target % self.qbits,
            vec![control % self.qbits]

        );

        self.gates.push(c_gate);
    }

    pub fn ccx(&mut self, control_1: usize, control_2: usize, target: usize) {
        assert_ne!(control_1, target);
        assert_ne!(control_2, target);
        assert_ne!(control_1, control_2);

        let c_gate : Gate = Gate::controlled(
            "Toffoli",
            "CCX",
            Matrix::X(),
            target % self.qbits,
            vec![control_1 % self.qbits, control_2 % self.qbits]

        );

        self.gates.push(c_gate);
    }

    pub fn swap(&mut self, q1: usize, q2: usize) {
        assert_ne!(q1, q2);

        self.cx(q1, q2);
        self.cx(q2, q1);
        self.cx(q1, q2);
    }

}