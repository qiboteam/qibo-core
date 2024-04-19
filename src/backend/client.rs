use std::io::{self, Error, Result};
use std::net::TcpStream;
use std::process::Command;

use crate::backend::message::{FromClient, FromServer};

use super::address::Address;

const PREFIX: &str = "qibo-backend";

#[derive(Debug)]
pub struct Client {
    address: Address,
    stream: Option<TcpStream>,
}

impl Client {
    /// Spawn a new server.
    pub fn spawn(name: &str) -> Result<Self> {
        use std::{thread, time};

        let executable = format!("{PREFIX}-{name}");

        let address = Address::new().map_err(|_| Error::from(io::ErrorKind::Other))?;
        println!("addr: {address}");
        println!("exec: {executable}");

        Command::new(executable).arg(&address.to_string()).spawn()?;
        // TODO: drop the sleep, by waiting on some kind of activation signal
        // e.g. it should try to open a connection, and close right after (or keep it)
        thread::sleep(time::Duration::from_millis(100));

        Self::connect(address)
    }

    /// Connect to existing server.
    pub fn connect(address: Address) -> Result<Self> {
        let mut client = Self {
            address,
            stream: None,
        };
        client.subscribe()?;
        Ok(client)
    }

    fn stream(&mut self) -> Result<&mut TcpStream> {
        if self.stream.is_none() {
            self.stream = Some(TcpStream::connect(&self.address.to_string())?);
        }
        Ok(self.stream.as_mut().unwrap())
    }

    pub fn subscribe(&mut self) -> Result<()> {
        FromClient::Subscribe.write(self.stream()?)?;
        Ok(())
    }

    pub fn execute(&mut self, circuit: &str) -> Result<String> {
        let mut stream = self.stream()?;
        FromClient::Something(circuit.to_owned()).write(&mut stream)?;

        let msg = FromServer::read(&mut stream)?;
        let FromServer::Reply(msg) = msg;

        self.close()?;
        Ok(msg)
    }

    pub fn close(&mut self) -> io::Result<()> {
        println!("Closing connection to backend {}", self.address);
        FromClient::Close.write(self.stream()?)?;
        self.stream = None;
        Ok(())
    }

    pub fn quit(&mut self) -> io::Result<()> {
        println!("Quitting backend server {}", self.address);
        FromClient::Quit.write(self.stream()?)?;
        self.stream = None;
        Ok(())
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        // TODO: attempt quitting the server
        // the server will hold a subscribers count, and just lower it by one
        // it will actually quit when it will reach 0
        if let Err(_) = self.quit() {
            println!("Failed closing backend {}", self.address);
        }
    }
}
