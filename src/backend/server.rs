use std::io::{Error, ErrorKind, Result};
use std::net::{TcpListener, TcpStream};

use super::address::Address;
use super::message::{FromClient, FromServer};

#[derive(Debug)]
pub struct Server {
    address: Address,
    clients: usize,
}

impl Server {
    pub fn new(address: &str) -> Result<Self> {
        Ok(Self {
            address: address
                .try_into()
                .map_err(|_| Error::new(ErrorKind::InvalidInput, ""))?,
            clients: 0,
        })
    }

    pub fn listen(&mut self) -> Result<()> {
        let listener = TcpListener::bind(&self.address.to_string())?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.handle_connection(stream)?,
                Err(_) => {
                    continue;
                    // TODO: handle failure (at least log)
                }
            };
            if self.clients == 0 {
                break;
            }
        }
        Ok(())
    }

    fn reply(stream: &mut TcpStream) -> Result<()> {
        FromServer::Reply("response from server".to_owned()).write(stream)?;
        Ok(())
    }

    fn handle_connection(&mut self, mut stream: TcpStream) -> Result<()> {
        loop {
            use FromClient::*;

            match FromClient::read(&mut stream)? {
                Subscribe => {
                    dbg!("subscribe");
                    self.clients += 1;
                }
                Something(msg) => {
                    dbg!(msg);
                    Self::reply(&mut stream)?;
                }
                Close => {
                    dbg!("close");
                    break;
                }
                Quit => {
                    dbg!("quit");
                    self.clients -= 1;
                    break;
                }
            }
        }
        Ok(())
    }
}
