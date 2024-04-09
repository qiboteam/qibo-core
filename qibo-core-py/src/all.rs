use pyo3::prelude::*;
use qibo_core::prelude::*;

#[pyclass]
pub struct PyGate {
    gate: Gate,
}

#[pymethods]
impl PyGate {
    #[new]
    pub fn new_gate(name: &str) -> PyGate {
        PyGate {
            gate: match name {
                "X" => (X {}).into(),
                "Y" => (Y {}).into(),
                "H" => (H {}).into(),
                _ => panic!("Unknown gate"),
            },
        }
    }
}
