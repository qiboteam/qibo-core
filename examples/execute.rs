use qibo_core::prelude::*;
use num::Complex;

fn main() {
    let mut c = Circuit::new(1);
    c.add(X.into(), vec![0]);
    c.add(H.into(), vec![0]);
    c.add(Y.into(), vec![0]);
    //c.add(RX(1.0).into(), vec![0]);

    let one = Complex::new(1., 0.);
    let zero = Complex::new(0., 0.);
    let mut state: Vec<Complex<f64>> = vec![0.4 * one, 0.2 * one];
    println!("{:?}\n", state);
    execute_circuit(c, &mut state);
    println!("{:?}", state);
}
