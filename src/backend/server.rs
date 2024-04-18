use std::io::{Error, ErrorKind, Result};
use std::net::{TcpListener, TcpStream};

use super::address::Address;
use super::message::Message;

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
            let quit = match stream {
                Ok(stream) => Self::handle_connection(stream)?,
                Err(_) => {
                    continue;
                    // TODO: handle failure (at least log)
                }
            };
            if quit {
                break;
            }
        }
        Ok(())
    }

    fn handle_connection(mut stream: TcpStream) -> Result<bool> {
        let quit = loop {
            match Message::read(&mut stream)? {
                Message::Something(msg) => {
                    dbg!(msg);
                }
                Message::Close => {
                    dbg!("closing...");
                    break false;
                }
                Message::Quit => {
                    dbg!("quitting...");
                    break true;
                }
            }
        };
        println!("connection interrupted");
        Ok(quit)
    }
}
