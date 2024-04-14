use pyo3::prelude::*;
use qibo_core::prelude::{self, More, One, Two};

#[pymodule]
pub mod gate {

    use super::*;

    #[pyclass]
    #[derive(Clone)]
    pub enum Gate {
        X {},
        Y {},
        RX { angle: f64 },
        CNOT {},
        TOFFOLI {},
    }

    impl Gate {
        pub(crate) fn gate(&self) -> prelude::Gate {
            match &self {
                &Self::X {} => One::X.into(),
                &Self::Y {} => One::Y.into(),
                &Self::RX { angle } => One::RX(*angle).into(),
                &Self::CNOT {} => Two::CNOT.into(),
                &Self::TOFFOLI {} => More::TOFFOLI.into(),
            }
        }
    }

    #[pymethods]
    impl Gate {
        #[getter]
        fn elements(&self) -> usize {
            self.gate().elements()
        }
    }
}
