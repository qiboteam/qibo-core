use pyo3::prelude::*;
use qibo_core::prelude;

#[pymodule]
pub mod gate {
    use super::*;

    #[pyfunction]
    pub fn X() -> Gate {
        Gate((prelude::X {}).into())
    }

    #[pyfunction]
    pub fn RX(angle: f64) -> Gate {
        Gate((prelude::RX(angle)).into())
    }

    #[pyfunction]
    pub fn CNOT() -> Gate {
        Gate((prelude::CNOT {}).into())
    }

    #[pyclass]
    pub struct Gate(prelude::Gate);

    #[pymethods]
    impl Gate {
        pub fn elements(&self) -> usize {
            self.0.elements()
        }
    }
}
