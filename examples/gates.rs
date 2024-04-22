use qibo_core::prelude::*;

fn main() {
    let gates: Vec<Gate> = vec![
        X.into(),
        RX(1.5).into(),
        TOFFOLI.into(),
        H.into(),
        CU1(3.0).into(),
    ];

    let serialized = serde_json::to_string(&gates).unwrap();
    println!("{:?}", serialized);

    let deserialized: Vec<Gate> = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
