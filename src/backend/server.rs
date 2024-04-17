use std::io::{Error, ErrorKind, Read, Result};
use std::net::{TcpListener, TcpStream};

use super::address::Address;

#[derive(Debug)]
pub struct Server {
    address: Address,
}

impl Server {
    pub fn new(address: &str) -> Result<Self> {
        Ok(Self {
            address: address
                .try_into()
                .map_err(|_| Error::new(ErrorKind::InvalidInput, ""))?,
        })
    }

    pub fn listen(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.address.to_string())?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => Self::handle_connection(stream)?,
                Err(_) => {
                    continue;
                    // TODO: handle failure (at least log)
                }
            };
        }
        Ok(())
    }

    fn read(stream: &mut TcpStream) -> Result<Vec<u8>> {
        let mut buffer = [0; 1024];
        let mut message = Vec::new();

        let mut len = [0; 8];
        stream.read_exact(&mut len)?;
        let len = u64::from_be_bytes(len);
        println!("{len}");

        while (message.len() as u64) < len {
            match stream.read(&mut buffer) {
                Ok(bytes_read) if bytes_read > 0 => {
                    println!("{:?}", &buffer[..bytes_read]);
                    message.extend_from_slice(&buffer[..bytes_read]);
                }
                Ok(_) | Err(_) => {
                    break;
                }
            }
        }
        Ok(message)
    }

    fn process(msg: Vec<u8>) -> bool {
        dbg!(msg);
        // TODO: break when close signal is received
        true
    }

    fn handle_connection(mut stream: TcpStream) -> Result<()> {
        loop {
            let msg = Self::read(&mut stream)?;
            if Self::process(msg) {
                println!("connection interrupted");
                break;
            }
        }
        Ok(())
    }
}
