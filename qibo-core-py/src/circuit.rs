use pyo3::prelude::*;
use qibo_core::prelude;

use crate::gate::gate::Gate;


#[pymodule]
pub mod circuit {
    use super::*;

    #[pyclass]
    struct Circuit {
        circuit: prelude::Circuit, 
        iteration_index: usize // index for iterating circuit queue
    }

    #[pymethods]
    impl Circuit {
        #[new]
        fn new(elements: usize) -> Self {
            Self { circuit: prelude::Circuit::new(elements), iteration_index: 0 }
        }

        fn add(&mut self, gate: Gate, elements: Vec<usize>) {
            self.circuit.add(gate.to_rust(), elements);
        }

        #[getter]
        fn n_elements(&self) -> usize {
            self.circuit.n_elements()
        }

        #[getter]
        fn measurements(&self) -> Vec<Gate> {
            vec![]
        }

        fn __iter__(mut slf: PyRefMut<Self>) -> PyRefMut<Self> {
            slf.iteration_index = 0; // reset iteration index
            slf
        }

        fn __next__(mut slf: PyRefMut<Self>) -> Option<(Gate, Vec<usize>)> {
            let gid = slf.iteration_index;
            if gid < slf.circuit.n_gates() {
                let gate = Gate::to_python(slf.circuit.gate(gid));
                let targets = slf.circuit.elements(gid);
                slf.iteration_index += 1;
                Some((gate, targets))
            } else {
                None
            }
        }

        fn __str__(&self) -> String {
            format!("{}", self.circuit)
        }
    }
}
