use std::io::Result;

use qibo_core::prelude::*;

fn main() -> Result<()> {
    let mut backend = Client::spawn("simple").expect("Backend not found.");
    let text = "ciao, come va?";
    println!("message: {text}");
    let res = backend.execute(&text)?;
    println!("response: {res}");
    Ok(())
}
