use std::io::{Error, ErrorKind, Result};
use std::net::{TcpListener, TcpStream};

use crate::circuit::Circuit;
use crate::state::State;

use super::address::Address;
use super::message::{FromClient, FromServer};

#[derive(Debug)]
pub struct Server {
    address: Address,
    clients: usize,
}

pub enum ToDo {
    Execute(Circuit, Option<State<usize>>),
    CloseConnection,
    QuitServer,
    Nothing,
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

    pub fn listen(&mut self) -> Result<TcpListener> {
        TcpListener::bind(&self.address.to_string())
    }

    pub fn receive(&mut self, stream: &mut TcpStream) -> Result<ToDo> {
        use FromClient::*;

        Ok(match FromClient::read(stream)? {
            Subscribe => {
                self.clients += 1;
                ToDo::Nothing
            }
            Execute(msg) => ToDo::Execute(Circuit::new(1), None),
            Close => ToDo::CloseConnection,
            Quit => {
                self.clients -= 1;
                if self.clients == 0 {
                    ToDo::QuitServer
                } else {
                    ToDo::CloseConnection
                }
            }
        })
    }

    pub fn send(stream: &mut TcpStream, result: Vec<u8>) -> Result<()> {
        FromServer::Result(result).write(stream)?;
        Ok(())
    }
}
