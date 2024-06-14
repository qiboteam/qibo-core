use std::io::Result;

use qibo_core::prelude::*;

fn main() -> Result<()> {
    let mut backend = Client::spawn("echo").expect("Backend not found.");
    let mut c = Circuit::new(5);
    c.add(X.into(), vec![2]);
    c.add(Y.into(), vec![2]);
    c.add(H.into(), vec![2]);
    c.add(CNOT.into(), vec![3, 1]);
    println!("circuit:\n{c}");
    let res = backend.execute::<usize>(&c, None)?;
    println!("response: {res:?}");
    Ok(())
}
