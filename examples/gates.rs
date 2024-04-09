use qibo_core::prelude::*;

fn main() {
    let gates: Vec<Gate> = vec![
        Gate::One(One::X),
        Gate::One(One::RX(1.5)),
        Gate::One(One::H),
    ];

    println!("{:#?}", gates);
}
