use core::panic;
use std::fmt::{self, Display};

use crate::gate::Gate;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Node {
    /// Gate ID
    gid: usize,
    /// Internal element
    element: usize,
}

#[derive(Debug, Clone, Copy)]
struct Edge(Option<Node>, Node);

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
    edges: Vec<Edge>,
    /// Current final gates of each wire
    ends: Vec<Option<Node>>,
}

impl Circuit {
    pub fn new(elements: usize) -> Self {
        Circuit {
            gates: vec![],
            edges: vec![],
            ends: vec![None; elements],
        }
    }

    pub fn add(&mut self, gate: Gate, elements: Vec<usize>) {
        self.gates.push(gate);
        // retrieve gate ID
        let gid = self.gates.len() - 1;
        for (i, &el) in elements.iter().enumerate() {
            let node = Node { gid, element: i };
            self.edges.push(Edge(self.ends[el], node));
            self.ends[el] = Some(node);
        }
    }

    fn previous(&self, node: Node) -> Option<Node> {
        for e in self.edges.iter() {
            if e.1 == node {
                return e.0;
            }
        }
        panic!("Gate not found")
    }

    pub fn wire(&self, element: usize) -> Vec<Gate> {
        let mut wire = vec![];
        let mut current = self.ends[element].clone();
        while current != None {
            let node = current.unwrap();
            let gate = self.gates[node.gid].clone();
            wire.push(gate);
            current = self.previous(node);
        }
        wire
    }

    pub fn elements(&self) -> usize {
        self.ends.len()
    }

    pub fn draw(&self) -> String {
        let mut wires = vec![];
        for i in 0..self.elements() {
            let mut wire: Vec<String> =
                self.wire(i).into_iter().map(|g| format!("{}", g)).collect();

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
