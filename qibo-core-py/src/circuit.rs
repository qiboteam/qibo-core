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

        #[getter]
        fn queue(&self) -> (Vec<Gate>, Vec<Vec<usize>>) {
            let queue = self.0.queue();
            let mut pygates = vec![];
            for &gate in queue.0.iter() {
                pygates.push(Gate::to_python(gate));
            }
            (pygates, queue.1)
        }

        fn __str__(&self) -> String {
            format!("{}", self.0)
        }
    }
}
