use crate::circuit::Circuit;

mod complex;
mod gate;
mod matrix;
mod circuit;

fn main() {
    let mut qc : Circuit = Circuit::new(2, 0);

    qc.h(0);
    qc.cx(0, 1);

    qc.compile();
    qc.run().print();
}
