//! Qibo core C bindings.

use std::ffi::{c_char, CStr, CString};
use std::slice;

use qibo_core::prelude::*;

#[no_mangle]
pub extern "C" fn qibo_core_circuit_new(n_elements: usize) -> Box<Circuit> {
    Box::new(Circuit::new(n_elements))
}

#[no_mangle]
pub extern "C" fn qibo_core_circuit_add(
    circuit: &mut Circuit,
    gate: &c_char,
    elements: &mut usize,
    n_elements: usize,
) {
    let elements = unsafe { slice::from_raw_parts_mut(elements, n_elements) }.to_vec();

    let gate = match (unsafe { CStr::from_ptr(gate) }).to_str().unwrap() {
        "H" => One::H.into(),
        "X" => One::X.into(),
        "Y" => One::Y.into(),
        "Z" => One::Z.into(),
        _ => One::X.into(),
    };

    circuit.add(gate, elements);
}

#[no_mangle]
pub extern "C" fn qibo_core_circuit_n_elements(circuit: &Circuit) -> usize {
    circuit.n_elements()
}

#[no_mangle]
pub extern "C" fn qibo_core_circuit_draw(circuit: &Circuit) -> *mut c_char {
    let repr = circuit.draw();

    CString::new(repr).unwrap().into_raw()
}
