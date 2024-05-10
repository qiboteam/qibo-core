use pyo3::prelude::*;

mod circuit;
mod gate;

#[pymodule]
mod qibo_core {
    use super::*;

    #[pymodule_export]
    use gate::gate;

    #[pymodule_export]
    use circuit::circuit;
}
