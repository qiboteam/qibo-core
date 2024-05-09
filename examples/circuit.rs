use qibo_core::prelude::*;

fn main() {
    let mut c = Circuit::new(5);
    c.add(X.into(), vec![2]);
    c.add(Y.into(), vec![2]);
    c.add(H.into(), vec![2]);
    let gid0 = c.add(CNOT.into(), vec![3, 1]);
    c.add(H.into(), vec![3]);
    let gid1 = c.add(CNOT.into(), vec![1, 4]);
    c.add(Y.into(), vec![4]);
    c.add(RX(3.14).into(), vec![0]);
    let gid2 = c.add(CNOT.into(), vec![2, 0]);

    println!("{}", c);
    println!("{:?}", c.elements(gid0));
    println!("{:?}", c.elements(gid1));
    println!("{:?}", c.elements(gid2));
    
    println!("\n");
    for (gate, elements) in c.gates_with_elements() {
        println!("{:?} {:?}", gate, elements);
    }
}
