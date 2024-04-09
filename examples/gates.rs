use qibo_core::*;

fn main() {
    let gates: Vec<Gate> = vec![(X {}).into(), (RX { theta: 1.5 }).into(), (H {}).into()];

    println!("{:#?}", gates);
}