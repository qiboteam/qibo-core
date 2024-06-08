use pyo3::prelude::*;
use qibo_core::prelude;

use crate::gate::gate::Gate;


#[pymodule]
pub mod circuit {
    use super::*;

    #[pyclass]
    struct Circuit(prelude::Circuit, usize);

    #[pymethods]
    impl Circuit {
        #[new]
        fn new(elements: usize) -> Self {
            Self(prelude::Circuit::new(elements), 0)
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

        fn __iter__(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
            slf.1 = 0;
            slf
        }

        fn __next__(mut slf: PyRefMut<Self>) -> Option<(Gate, Vec<usize>)> {
            let gid = slf.1;
            if gid < slf.0.n_gates() {
                let gate = Gate::to_python(slf.0.gates(gid));
                let targets = slf.0.elements(gid);
                slf.1 += 1;
                Some((gate, targets))
            } else {
                None
            }
        }

        fn __str__(&self) -> String {
            format!("{}", self.0)
        }
    }
}
