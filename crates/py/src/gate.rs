use pyo3::prelude::*;
use qibo_core::prelude::{self, One, Two};

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
        M {},
        RX { angle: f64 },
        U1 { angle: f64 },
        SWAP {},
    }

    impl Gate {
        /// Convert Python gates to Rust gates
        pub(crate) fn to_rust(&self) -> prelude::Gate {
            match &self {
                &Self::H {} => One::H.into(),
                &Self::X {} => One::X.into(),
                &Self::Y {} => One::Y.into(),
                &Self::Z {} => One::Z.into(),
                &Self::M {} => One::M.into(),
                &Self::RX { angle } => One::RX(*angle).into(),
                &Self::U1 { angle } => One::U1(*angle).into(),
                &Self::SWAP {} => Two::SWAP.into(),
            }
        }

        /// Convert Rust gates to Python
        pub fn to_python(gate: prelude::Gate) -> Self {
            match gate {
                prelude::Gate::One(One::H) => Self::H {},
                prelude::Gate::One(One::X) => Self::X {},
                prelude::Gate::One(One::Y) => Self::Y {},
                prelude::Gate::One(One::Z) => Self::Z {},
                prelude::Gate::One(One::M) => Self::M {},
                prelude::Gate::One(One::RX(angle)) => Self::RX { angle },
                prelude::Gate::One(One::U1(angle)) => Self::U1 { angle },
                prelude::Gate::Two(Two::SWAP) => Self::SWAP {},
                _ => todo!()
            }
        }
    }

    #[pymethods]
    impl Gate {
        #[getter]
        fn targets(&self) -> usize {
            self.to_rust().targets()
        }
    }
}
