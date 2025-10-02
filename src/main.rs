use crate::circuit::Circuit;

mod complex;
mod gate;
mod matrix;
mod circuit;

fn main() {
    let mut qc : Circuit = Circuit::new(4, 0);

    for q in qc.iter() {qc.h(q)};
    for q in qc.iter() {qc.cx(q, q + 1);}
    for q in qc.iter() {qc.h(q)};

    qc.compile();
    qc.run();
}
