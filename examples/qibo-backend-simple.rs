use std::env;
use std::io::Result;

use qibo_core::prelude::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let server = Server::new(&args[1])?;
    server.listen()?;
    dbg!(server);
    Ok(())
}
