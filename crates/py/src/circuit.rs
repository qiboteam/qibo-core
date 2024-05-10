use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use qibo_core::prelude;

use crate::gate::gate::Gate;

#[pymodule]
pub mod circuit {
    use super::*;

    #[pyclass(sequence)]
    struct Circuit(prelude::Circuit);

    #[pymethods]
    impl Circuit {
        #[new]
        fn new(elements: usize) -> Self {
            Self(prelude::Circuit::new(elements))
        }

        fn add(&mut self, gate: Gate, elements: Vec<usize>) {
            self.0.add(gate.to_rust(), elements);
        }

        #[getter]
        fn n_elements(&self) -> usize {
            self.0.n_elements()
        }

        #[getter]
        fn measurements(&self) -> Vec<Gate> {
            vec![]
        }

        fn __getitem__(&self, gid: usize) -> PyResult<(Gate, Vec<usize>)> {
            if gid >= self.0.n_gates() {
                return Err(PyIndexError::new_err(""));
            }
            let gate = Gate::to_python(self.0.gate(gid));
            let targets = self.0.elements(gid);
            Ok((gate, targets))
        }

        fn __str__(&self) -> String {
            format!("{}", self.0)
        }
    }
}
