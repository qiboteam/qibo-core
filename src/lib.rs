//! Qibo framework core elements
pub mod circuit;
pub mod gate;
pub mod prelude;


// Rust to C++ interface with cxx.rs
use circuit::Circuit;
use gate::Gate;
use gate::One;
use cxx::CxxVector;

#[cxx::bridge(namespace = "qibo")]
mod ffi {
    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type Gate;
        type Circuit;
        type One;
        fn make_circuit(n_elements: usize) -> Box<Circuit>;
        fn add_gate(self: &mut Circuit, gate: Box<Gate>, elements: &CxxVector<usize>) -> usize;
        fn draw(self: &Circuit) -> String;
        fn make_x_gate() -> Box<Gate>;
    }
}
fn make_x_gate() -> Box<Gate> {
    Box::new(One::X.into())
}

fn make_circuit(n_elements: usize) -> Box<Circuit> {
    Box::new(Circuit::new(n_elements))
}

impl Circuit {
    fn add_gate(&mut self, gate: Box<Gate>, elements: &CxxVector<usize>) -> usize {
        let mut gids: Vec<usize> = Vec::new();
        for x in elements {
            gids.push(*x);
        }
        self.add(*gate, gids)
    }
}
