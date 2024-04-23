use pyo3::prelude::*;

mod backend;
mod circuit;
mod gate;

#[pymodule]
mod qibo_core {
    use super::*;

    #[pymodule_export]
    use gate::gate;

    #[pymodule_export]
    use circuit::circuit;

    #[pymodule_export]
    use backend::backend;
}
