use qibo_core::prelude::*;

fn main() {
    let mut circuit = Circuit::new(5);
    circuit.add(X.into(), vec![2]);
    circuit.add(Y.into(), vec![2]);
    circuit.add(H.into(), vec![2]);
    circuit.add(CNOT.into(), vec![2, 1]);
    circuit.add(Y.into(), vec![4]);

    println!("{}", circuit);
}
