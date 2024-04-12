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

    pub fn add(&mut self, gate: Gate, elements: Vec<usize>) -> usize {
        self.gates.push(gate);
        // retrieve gate ID
        let gid = self.gates.len() - 1;
        for (i, &el) in elements.iter().enumerate() {
            let node = Node { gid, element: i };
            self.edges.push(Edge(self.ends[el], node));
            self.ends[el] = Some(node);
        }
        gid
    }

    fn previous(&self, node: Node) -> Result<Option<Node>, ()> {
        for e in self.edges.iter() {
            if e.1 == node {
                return Ok(e.0);
            }
        }
        Err(())
    }

    /// Return the next node in the wire
    ///
    /// If the node is the last one in the wire `Ok(None)` is returned. If the node is not found,
    /// `Err(())` is returned.
    fn next(&self, node: Node) -> Result<Option<Node>, ()> {
        for end in self.ends.iter() {
            if *end == Some(node) {
                return Ok(None);
            }
        }

        for e in self.edges.iter() {
            if e.0 == Some(node) {
                return Ok(Some(e.1));
            }
        }
        Err(())
    }

    pub fn wire(&self, element: usize) -> Vec<Gate> {
        let mut wire = vec![];
        let mut current = self.ends[element].clone();
        while current != None {
            let node = current.unwrap();
            let gate = self.gates[node.gid].clone();
            wire.push(gate);
            current = self.previous(node).expect("Gate not found");
        }
        wire.into_iter().rev().collect()
    }

    pub fn elements(&self) -> usize {
        self.ends.len()
    }

    pub fn wires(&self) -> Vec<Vec<Gate>> {
        (0..self.elements()).map(|i| self.wire(i)).collect()
    }

    fn target(&self, node: Node) -> Option<usize> {
        let mut current = Some(node);
        while current != None {
            match self.next(current.unwrap()) {
                Ok(next @ Some(_)) => {
                    current = next;
                }
                Ok(None) => {
                    break;
                }
                Err(()) => {
                    return None;
                }
            }
        }
        self.ends.iter().position(|x| *x == current)
    }

    pub fn targets(&self, gid: usize) -> Vec<usize> {
        (0..self.gates[gid].elements())
            .map(|element| self.target(Node { gid, element }).expect("Dangling gate"))
            .collect()
    }

    pub fn draw(&self) -> String {
        self.wires()
            .iter()
            .enumerate()
            .map(|(i, w)| {
                format!("{}: ", i)
                    + &w.iter()
                        .map(|g| format!("{}", g))
                        .collect::<Vec<_>>()
                        .join("-")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.draw())
    }
}
