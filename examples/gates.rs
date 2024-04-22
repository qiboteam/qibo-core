use qibo_core::prelude::*;

use serde::Serialize;

fn main() {
    let gates: Vec<Gate> = vec![X.into(), RX(1.5).into(), H.into()];

    let serialized = serde_json::to_string(&gates).unwrap();
    println!("{:#?}", gates);
}
