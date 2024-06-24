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
        "CNOT" => Two::CNOT.into(),
        _ => One::X.into(),
    };

    circuit.add(gate, elements);
}

#[no_mangle]
pub extern "C" fn qibo_core_circuit_n_elements(circuit: &Circuit) -> usize {
    circuit.n_elements()
}

#[no_mangle]
pub extern "C" fn qibo_core_circuit_n_gates(circuit: &Circuit) -> usize {
    circuit.n_gates()
}

#[no_mangle]
pub extern "C" fn qibo_core_circuit_gate(circuit: &Circuit, gid: usize) -> *const c_char {
    let gate = circuit.gate(gid);
    let name = match gate {
        Gate::One(One::H) => "H",
        Gate::One(One::X) => "X",
        Gate::One(One::Y) => "Y",
        Gate::One(One::Z) => "Z",
        Gate::Two(Two::CNOT) => "CNOT",
        _ => todo!()
    };

    CString::new(name).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn qibo_core_circuit_elements(circuit: &Circuit, gid: usize, ptr: *mut *const usize, len: *mut usize) {
    let elements = circuit.elements(gid);
    // Complaints about what follows are to be directed to ChatGPT
    let boxed_slice = elements.clone().into_boxed_slice();
    unsafe {
        *ptr = boxed_slice.as_ptr();
        *len = elements.len();
    }
    std::mem::forget(boxed_slice);
}

#[no_mangle]
pub extern "C" fn qibo_core_circuit_free_elements(ptr: *const usize, len: usize) {
    if !ptr.is_null() {
        unsafe {
            let _ = Vec::from_raw_parts(ptr as *mut usize, len, len);
        }
    }
}

#[no_mangle]
pub extern "C" fn qibo_core_circuit_draw(circuit: &Circuit) -> *mut c_char {
    let repr = circuit.draw();

    CString::new(repr).unwrap().into_raw()
}
