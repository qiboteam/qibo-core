use core::panic;
use std::fmt::{self, Display};

use crate::gate::Gate;

/// A discrete gate-based representation of a quantum computation.
///
/// The circuit is represented as an unstrtuctured set of gates, with their connectivity separately
/// recorded by an adjacency list.
/// Moreover, a circuit is not a random graph, but rather a set of wires, with possible links
/// across them. This is represented by recording the circuit ends, where it is possible to append
/// further gates, including measurements. They identify the quantum elements (local subsystem)
/// where the gates are acting on.
#[derive(Debug)]
pub struct Circuit {
    /// Set of gates
    gates: Vec<Gate>,
    /// Gates connectivity
    edges: Vec<(Option<usize>, usize)>,
    /// Current final gates of each wire
    ends: Vec<Option<usize>>,
}

impl Circuit {
    pub fn new(elements: usize) -> Self {
        Circuit {
            gates: vec![],
            edges: vec![],
            ends: vec![None; elements],
        }
    }

    pub fn add(&mut self, gate: Gate, element: usize) {
        self.gates.push(gate);
        // retrieve gate ID
        let gid = self.gates.len() - 1;
        self.edges.push((self.ends[element], gid));
        self.ends[element] = Some(gid);
    }

    fn previous(&self, gid: usize) -> Option<usize> {
        for e in self.edges.iter() {
            if e.1 == gid {
                return e.0;
            }
        }
        panic!("Gate not found")
    }

    pub fn wire(&self, element: usize) -> Vec<&Gate> {
        let mut wire = vec![];
        let mut cur = self.ends[element].clone();
        while cur != None {
            let gid = cur.unwrap();
            wire.push(&self.gates[gid]);
            cur = self.previous(gid);
        }
        wire
    }

    pub fn elements(&self) -> usize {
        self.ends.len()
    }

    pub fn draw(&self) -> String {
        let mut wires = vec![];
        for i in 0..self.elements() {
            let mut wire: Vec<String> = self
                .wire(i)
                .into_iter()
                .map(|g| format!("{:?}", g))
                .collect();

            wire.push(format!("{} ", i));
            wires.push(
                wire.iter()
                    .rev()
                    .map(|x| x.as_str())
                    .collect::<Vec<&str>>()
                    .join("-"),
            );
        }
        wires.join("\n")
    }
}

impl Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.draw())
    }
}
