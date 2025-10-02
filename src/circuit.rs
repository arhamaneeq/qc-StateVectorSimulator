use crate::{gate::Gate, matrix::Matrix};

struct Circuit {
    qbits: usize,
    cbits: usize,
    gates: Vec<Gate>
}

impl Circuit {
    pub fn new(_qbits: usize, _cbits: usize) -> Self {
        Self {
            qbits: _qbits,
            cbits: _cbits,
            gates: Vec::new(),
        }
    }

    pub fn h(&mut self, qbit: usize) {
        let h_gate: Gate = Gate::new(
            "Hadamard",
            "H", 
            Matrix::H(), 
            vec![qbit % self.qbits],
        );

        self.gates.push(h_gate);
    }

    pub fn x(&mut self, qbit: usize) {
        let x_gate: Gate = Gate::new(
            "Pauli X",
            "X",
            Matrix::X(),
            vec![qbit % self.qbits],
        );

        self.gates.push(x_gate);
    }

    pub fn y(&mut self, qbit: usize) {
        let y_gate: Gate = Gate::new(
            "Pauli Y",
            "Y",
            Matrix::Y(),
            vec![qbit % self.qbits]
        );

        self.gates.push(y_gate);
    }

    pub fn z(&mut self, qbit: usize) {
        let z_gate: Gate = Gate::new(
            "Pauli Z",
            "Z",
            Matrix::Z(),
            vec![qbit % self.qbits],
        );

        self.gates.push(z_gate);
    }

    pub fn rx(&mut self, theta: f32, qbit: usize) {
        let r_gate: Gate = Gate::new(
            "Rotation X",
            "Rx",
            Matrix::Rx(theta),
            vec![qbit % self.qbits]
        );

        self.gates.push(r_gate);
    }

    pub fn ry(&mut self, theta: f32, qbit: usize) {
        let r_gate: Gate = Gate::new(
            "Rotation Y",
            "Ry",
            Matrix::Ry(theta),
            vec![qbit % self.qbits]
        );

        self.gates.push(r_gate);
    }

    pub fn rz(&mut self, theta: f32, qbit: usize) {
        let r_gate: Gate = Gate::new(
            "Rotation Z",
            "Rz",
            Matrix::Rz(theta),
            vec![qbit % self.qbits]
        );

        self.gates.push(r_gate);
    }

    pub fn cx(&mut self, control: usize, target: usize) {
        assert_ne!(control, target);

        let cx_gate : Gate = Gate::new(
            "Controlled X",
            "CX",
            Matrix::proj0() ^ Matrix::I() + Matrix::proj1() ^ Matrix::X(),
            vec![control, target]
        );

        self.gates.push(cx_gate);
    }



}