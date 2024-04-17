use std::net::{TcpListener, TcpStream};

use super::address::Address;

#[derive(Debug)]
pub struct Server {
    address: Address,
}

impl Server {
    pub fn listen(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.address.to_string())?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    Self::handle_connection(stream);
                }
                Err(_) => {
                    // TODO: handle failure (at least log)
                }
            }
        }
        Ok(())
    }

    fn handle_connection(stream: TcpStream) {
        println!("{stream:?}");
    }
}
