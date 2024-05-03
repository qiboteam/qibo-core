use std::env;
use std::io::Result;

use qibo_core::prelude::*;

fn execute(circuit: Circuit, state: Option<State<usize>>) -> Vec<u8> {
    vec![]
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Expected argument '<address>:<port>'.");
        return Ok(());
    }

    let mut server = Server::new(&args[1])?;

    let listener = server.listen()?;
    for stream in listener.incoming() {
        if match stream {
            Ok(mut stream) => loop {
                match server.receive(&mut stream)? {
                    ToDo::CloseConnection => break false,
                    ToDo::QuitServer => break true,
                    ToDo::Execute(circuit, state) => {
                        Server::send(&mut stream, execute(circuit, state))?;
                    }
                    ToDo::Nothing => {}
                }
            },
            Err(_) => {
                continue;
                // TODO: handle failure (at least log)
            }
        } {
            break;
        };
    }

    Ok(())
}
