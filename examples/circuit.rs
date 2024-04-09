use qibo_core::prelude::*;

fn main() {
    let mut circuit = Circuit::new(5);
    circuit.add(Gate::One(One::X), 2);
    circuit.add(Gate::One(One::Y), 2);
    circuit.add(Gate::One(One::H), 2);
    circuit.add(Gate::One(One::Y), 4);

    println!("{}", circuit);
}
