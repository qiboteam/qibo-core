use std::io::Result;

use qibo_core::prelude::*;

fn main() -> Result<()> {
    let mut backend = Client::spawn("simple").expect("Backend not found.");
    let res = backend.execute("ciao, come va?")?;
    println!("response: {}", res);
    Ok(())
}
