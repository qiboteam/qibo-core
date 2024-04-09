use pyo3::prelude::*;

mod all;

use all::*;

#[pymodule]
fn qibo_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Gate>()?;
    Ok(())
}
