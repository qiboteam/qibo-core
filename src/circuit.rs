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
/// The circuit is represented as an unstructured set of gates, with their connectivity separately
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
    pub fn new(n_elements: usize) -> Self {
        Circuit {
            gates: vec![],
            edges: vec![],
            ends: vec![None; n_elements],
        }
    }

    pub fn add(&mut self, gate: Gate, elements: Vec<usize>) -> usize {
        self.gates.push(gate);
        // retrieve gate ID
        let gid = self.gates.len() - 1;
        for &el in elements.iter() {
            let node = Node { gid, element: el };
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

    pub fn gates_with_elements(&self) -> impl Iterator<Item = (Gate, Vec<usize>)> {
        let mut gates = vec![];
        let mut elements = vec![];
        let mut gid = 0;
        for &edge in self.edges.iter() {
            let node = edge.1;
            if node.gid == gid {
                elements.push(node.element);
            } else {
                gates.push((self.gates[gid], elements));
                gid = node.gid;
                elements = vec![node.element];
            }
        }
        if self.gates[gid].elements() > 1 {
            gates.push((self.gates[gid], elements));
        }
        gates.into_iter()
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

    pub fn n_elements(&self) -> usize {
        self.ends.len()
    }

    pub fn wires(&self) -> Vec<Vec<Gate>> {
        (0..self.n_elements()).map(|i| self.wire(i)).collect()
    }

    /// Determine the elements the specified gate is acting on.
    pub fn elements(&self, gid: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter_map(|&edge| match edge.1.gid {
                g if g == gid => Some(edge.1.element),
                _ => None,
            })
            .collect()
    }

    pub fn draw(&self) -> String {
        let mut wires: Vec<String> = (0..self.n_elements()).map(|i| format!("q{i}: ")).collect();

        for (gid, gate) in self.gates.iter().enumerate() {
            match gate {
                &Gate::One(_) => {
                    wires[self.elements(gid)[0]] += &format!("{SEG}{gate}");
                }
                _ => {
                    pad(&mut wires);
                    let elements = self.elements(gid);
                    let (up, down) = (
                        elements.iter().min().unwrap(),
                        elements.iter().max().unwrap(),
                    );
                    for w in 0..self.n_elements() {
                        wires[w] += &(if elements[..gate.targets()].contains(&w) {
                            format!("{SEG}{gate}")
                        } else if w < *up || w > *down {
                            format!("{SEG}{SEG}")
                        } else if elements.iter().position(|x| *x == w) == None {
                            format!("{SEG}|")
                        } else {
                            format!("{SEG}o")
                        })
                    }
                    pad(&mut wires);
                }
            }
        }
        pad(&mut wires);
        wires.iter_mut().for_each(|w| w.push_str(SEG));

        wires.join("\n")
    }
}

const SEG: &str = "-";

fn pad(wires: &mut Vec<String>) {
    let length = wires.iter().map(|w| w.len()).max().unwrap();
    for wire in wires.iter_mut() {
        wire.push_str(&SEG.repeat(length.saturating_sub(wire.len())))
    }
}

impl Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.draw())
    }
}
