use qibo_core::prelude::*;

fn main() {
    let mut c = Circuit::new(5);
    c.add(X.into(), vec![2]);
    c.add(Y.into(), vec![2, 0]);
    c.add(H.into(), vec![1, 4]);
    let gid = c.add(X.into(), vec![3, 1]);
    c.add(H.into(), vec![3, 1, 4]);
    c.add(X.into(), vec![1, 4]);
    c.add(Y.into(), vec![4]);
    c.add(RX(3.14).into(), vec![0]);
    c.add(M.into(), vec![1]);
    c.add(M.into(), vec![3]);
    c.add(M.into(), vec![4]);

    println!("{}\n", c);

    println!("{:?}\n", c.elements(gid));

    println!("{:?}\n", c.measured_elements());

    for gid in 0..c.n_gates() {
        println!("{:?}", c.elements(gid));
    }
}
