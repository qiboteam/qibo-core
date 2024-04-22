use std::io::{self, Error, Result};
use std::net::TcpStream;
use std::process::Command;

use super::address::Address;
use super::message::{FromClient, FromServer};
use crate::circuit::Circuit;

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

    pub fn execute(&mut self, circuit: &Circuit) -> Result<String> {
        let mut stream = self.stream()?;
        FromClient::Something(serde_json::to_string(circuit)?).write(&mut stream)?;

        let msg = FromServer::read(&mut stream)?;
        let FromServer::Reply(msg) = msg;

        self.close()?;
        Ok(msg)
    }

    pub fn close(&mut self) -> io::Result<()> {
        FromClient::Close.write(self.stream()?)?;
        self.stream = None;
        Ok(())
    }

    pub fn quit(&mut self) -> io::Result<()> {
        FromClient::Quit.write(self.stream()?)?;
        self.stream = None;
        Ok(())
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if let Err(_) = self.quit() {
            println!("Failed closing backend {}", self.address);
        }
    }
}
