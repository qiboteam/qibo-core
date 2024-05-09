use qibo_core::prelude::*;

fn main() {
    let gates: Vec<Gate> = vec![X.into(), Y.into(), RX(1.5).into(), H.into()];
    println!("{:#?}\n", gates);

    for gate in gates.iter() {
        println!("{:?}", to_matrix(*gate));
    }
}