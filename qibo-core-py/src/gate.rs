use pyo3::prelude::*;
use qibo_core::prelude;

#[pymodule]
pub mod gate {
    use super::*;

    #[pyfunction]
    fn X() -> Gate {
        Gate((prelude::X {}).into())
    }

    #[pyfunction]
    fn RX(angle: f64) -> Gate {
        Gate((prelude::RX(angle)).into())
    }

    #[pyfunction]
    fn CNOT() -> Gate {
        Gate((prelude::CNOT {}).into())
    }

    #[pyclass]
    #[derive(Clone)]
    pub struct Gate(pub(crate) prelude::Gate);

    #[pymethods]
    impl Gate {
        fn elements(&self) -> usize {
            self.0.elements()
        }
    }
}
