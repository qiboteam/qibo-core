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
    edges: Vec<(usize, usize)>,
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
}
