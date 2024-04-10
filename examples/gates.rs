use qibo_core::prelude::*;

fn main() {
    let gates: Vec<Gate> = vec![X.into(), RX(1.5).into(), H.into()];

    println!("{:#?}", gates);
}
