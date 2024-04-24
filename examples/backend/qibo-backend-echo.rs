use std::env;
use std::io::Result;

use qibo_core::prelude::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Expected argument '<address>:<port>'.");
        return Ok(());
    }
    let mut server = Server::new(&args[1])?;
    server.listen()?;
    Ok(())
}
