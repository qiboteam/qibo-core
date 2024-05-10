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
        /// Convert Python gates to Rust gates
        pub(crate) fn to_rust(&self) -> prelude::Gate {
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

        /// Convert Rust gates to Python
        pub fn to_python(gate: prelude::Gate) -> Self {
            match gate {
                prelude::Gate::One(One::H) => Self::H {},
                prelude::Gate::One(One::X) => Self::X {},
                prelude::Gate::One(One::Y) => Self::Y {},
                prelude::Gate::One(One::Z) => Self::Z {},
                prelude::Gate::One(One::RX(angle)) => Self::RX { angle },
                prelude::Gate::Two(Two::CNOT) => Self::CNOT {},
                prelude::Gate::Two(Two::CU1(angle)) => Self::CU1 { angle },
                prelude::Gate::Two(Two::SWAP) => Self::SWAP {},
                prelude::Gate::More(More::TOFFOLI) => Self::TOFFOLI {},
                _ => todo!()
            }
        }
    }

    #[pymethods]
    impl Gate {
        #[getter]
        fn elements(&self) -> usize {
            self.to_rust().elements()
        }
    }
}
