//! Qibo core C bindings.

use qibo_core::prelude::*;

#[no_mangle]
pub extern "C" fn qibo_core_circuit_new(n_elements: usize) -> Box<Circuit> {
    Box::new(Circuit::new(n_elements))
}
