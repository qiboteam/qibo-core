use qibo_core::prelude::*;

fn main() {
    let mut circuit = Circuit::new(5);
    circuit.add((X {}).into(), 2);
    circuit.add((Y {}).into(), 2);
    circuit.add((Y {}).into(), 4);

    println!("{}", circuit);
}
