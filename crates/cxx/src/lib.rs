// Rust to C++ interface with cxx.rs
use cxx::CxxVector;
use qibo_core::prelude as qibo_;

struct Gate(qibo_::Gate);
struct Circuit(qibo_::Circuit);

#[cxx::bridge(namespace = "qibo_core")]
mod ffi {
    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type Gate;
        type Circuit;
        fn make_circuit(n_elements: usize) -> Box<Circuit>;
        fn add(self: &mut Circuit, gate: Box<Gate>, elements: &CxxVector<usize>) -> usize;
        fn draw(self: &Circuit) -> String;
        fn make_x_gate() -> Box<Gate>;
    }
}
fn make_x_gate() -> Box<Gate> {
    Box::new(Gate(qibo_::One::X.into()))
}

fn make_circuit(n_elements: usize) -> Box<Circuit> {
    Box::new(Circuit(qibo_::Circuit::new(n_elements)))
}

impl Circuit {
    fn add(&mut self, gate: Box<Gate>, elements: &CxxVector<usize>) -> usize {
        let mut gids: Vec<usize> = Vec::new();
        for x in elements {
            gids.push(*x);
        }
        self.0.add(gate.0, gids)
    }

    fn draw(&self) -> String {
        self.0.draw()
    }
}
