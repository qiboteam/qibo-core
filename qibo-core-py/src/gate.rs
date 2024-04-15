use pyo3::prelude::*;
use qibo_core::prelude::{self, More, One, Two};

#[pymodule]
pub mod gate {

    use super::*;

    #[pyclass]
    #[derive(Clone)]
    pub enum Gate {
        H {},
        X {},
        Y {},
        Z {},
        RX { angle: f64 },
        CNOT {},
        CU1 { angle: f64 },
        SWAP {},
        TOFFOLI {},
    }

    impl Gate {
        pub(crate) fn gate(&self) -> prelude::Gate {
            match &self {
                &Self::H {} => One::H.into(),
                &Self::X {} => One::X.into(),
                &Self::Y {} => One::Y.into(),
                &Self::Z {} => One::Z.into(),
                &Self::RX { angle } => One::RX(*angle).into(),
                &Self::CNOT {} => Two::CNOT.into(),
                &Self::CU1 { angle } => Two::CU1(*angle).into(),
                &Self::SWAP {} => Two::SWAP.into(),
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
