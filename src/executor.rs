use num::complex::Complex;

use crate::gate::Gate;
use crate::gate::One;
use crate::gate::Two;
use crate::circuit::Circuit;


fn rx_matrix(theta: f64) -> [Complex<f64>; 4] {
    let a = Complex::new(f64::cos(theta / 2.), 0.);
    let b = Complex::new(0., -f64::sin(theta / 2.));
    [a, b, b, a]
}

fn u1_matrix(theta: f64) -> [Complex<f64>; 4] {
    let one = Complex::new(1., 0.);
    let zero = Complex::new(0., 0.);
    let eith = Complex::new(f64::cos(theta), f64::sin(theta));
    [one, zero, zero, eith]
}

pub fn to_matrix(gate: Gate) -> [Complex<f64>; 4] {
    let one = Complex::new(1., 0.);
    let zero = Complex::new(0., 0.);
    let h = Complex::new(1.0 / f64::sqrt(2.0), 0.);
    let j = Complex::new(0., 1.);
    
    match gate {
        Gate::One(One::H) => [h, h, h, -h],
        Gate::One(One::X) | Gate::Two(Two::CNOT) => [zero, one, one, zero],
        Gate::One(One::Y) => [zero, -j, j, zero],
        Gate::One(One::Z) => [one, zero, zero, -one],
        Gate::One(One::RX(theta)) => rx_matrix(theta),
        Gate::Two(Two::CU1(theta)) => u1_matrix(theta),
        _ => todo!(),
    }
}


fn apply_gate(state: &mut Vec<Complex<f64>>, gate: [Complex<f64>; 4], target: usize, nqubits: usize) {
    let m = nqubits - target - 1;
    let nstates = 1 << (nqubits - 1);
    let tk = 1 << m;
    // TODO: This needs to be parallelized for large number of qubits
    for g in 0..nstates {
        let i1 = ((g >> m) << (m + 1)) + (g & (tk - 1));
        let i2 = i1 + tk;
        let state1 = state[i1];
        let state2 = state[i2];
        state[i1] = gate[0] * state1 + gate[1] * state2;
        state[i2] = gate[2] * state1 + gate[3] * state2;
    }
}


pub fn execute_circuit(circuit: Circuit, state: &mut Vec<Complex<f64>>) {
    for (gate, elements) in circuit.gates_with_elements() {
        let matrix = to_matrix(gate);
        // TODO: Multiqubit gates
        apply_gate(state, matrix, elements[0], circuit.n_elements())
    }
}
