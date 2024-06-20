//! Qibo core C bindings.

use std::ffi::{c_char, CStr, CString};

use qibo_core::prelude::*;

#[no_mangle]
pub extern "C" fn qibo_core_circuit_new(n_elements: usize) -> Box<Circuit> {
    Box::new(Circuit::new(n_elements))
}

#[no_mangle]
pub extern "C" fn qibo_core_circuit_add(mut circuit: Box<Circuit>, gate: *const c_char) {
    let gate = match (unsafe { CStr::from_ptr(gate) }).to_str().unwrap() {
        "H" => One::H.into(),
        "X" => One::X.into(),
        "Y" => One::Y.into(),
        "Z" => One::Z.into(),
        _ => One::X.into(),
    };
    println!("{gate:?}");

    circuit.add(gate, vec![0]);
}

#[no_mangle]
pub extern "C" fn qibo_core_circuit_n_elements(circuit: Box<Circuit>) -> usize {
    circuit.n_elements()
}

#[no_mangle]
pub extern "C" fn qibo_core_circuit_draw(circuit: Box<Circuit>) -> *mut c_char {
    let repr = circuit.draw();
    CString::new(repr).unwrap().into_raw()
}
