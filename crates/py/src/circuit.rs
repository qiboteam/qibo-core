use pyo3::prelude::*;
use qibo_core::prelude;

use crate::gate::gate::Gate;

#[pymodule]
pub mod circuit {
    use super::*;

    #[pyclass]
    struct Circuit(prelude::Circuit);

    #[pymethods]
    impl Circuit {
        #[new]
        fn new(elements: usize) -> Self {
            Self(prelude::Circuit::new(elements))
        }

        fn add(&mut self, gate: Gate, elements: Vec<usize>) {
            self.0.add(gate.gate(), elements);
        }

        #[getter]
        fn n_elements(&self) -> usize {
            self.0.n_elements()
        }

        fn __str__(&self) -> String {
            format!("{}", self.0)
        }
    }
}
