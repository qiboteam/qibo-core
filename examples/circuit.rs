use qibo_core::prelude::*;

fn main() {
    let mut c = Circuit::new(5);
    c.add(X.into(), vec![2]);
    c.add(Y.into(), vec![2]);
    c.add(H.into(), vec![2]);
    c.add(CNOT.into(), vec![3, 1]);
    c.add(H.into(), vec![3]);
    let gid = c.add(CNOT.into(), vec![1, 4]);
    c.add(Y.into(), vec![4]);
    c.add(RX(3.14).into(), vec![0]);

    println!("{}\n", c);

    let serialized = serde_json::to_string(&c).unwrap();
    println!("{}", serialized);
    let deserialized: Circuit = serde_json::from_str(&serialized).unwrap();
    println!("deserialized =\n{}\n", deserialized);

    println!("{:?}", c.elements(gid));
}
