use pyo3::prelude::*;

mod all;

#[pymodule]
mod qibo_core {
    use super::*;

    #[pymodule_export]
    use all::gate;
}
