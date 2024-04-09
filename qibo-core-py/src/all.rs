use pyo3::prelude::*;
use qibo_core::prelude;

#[pyclass]
pub struct Gate(prelude::Gate);

#[pymethods]
impl Gate {
    #[new]
    pub fn new_gate(name: &str) -> Gate {
        Gate(match name {
            "X" => (prelude::X {}).into(),
            "Y" => (prelude::Y {}).into(),
            "H" => (prelude::H {}).into(),
            _ => panic!("Unknown gate"),
        })
    }
}
