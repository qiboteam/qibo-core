use std::io::{self, Error, Result};
use std::net::TcpStream;
use std::process::Command;

use crate::backend::message::Message;

use super::address::Address;

const PREFIX: &str = "qibo-backend";

#[derive(Debug)]
pub struct Client {
    address: Address,
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

        let client = Self { address };
        client.subscribe()?;

        Ok(client)
    }

    /// Connect to existing server.
    pub fn connect(address: Address) -> Self {
        Self { address }
    }

    fn stream(&self) -> io::Result<TcpStream> {
        TcpStream::connect(&self.address.to_string())
    }

    pub fn subscribe(&self) -> Result<()> {
        Message::Subscribe.write(&mut self.stream()?)?;
        Ok(())
    }

    pub fn execute(&self, circuit: &str) -> Result<String> {
        // TODO: hold the stream, and avoid reinitializing it if the connection is open
        let mut stream = self.stream()?;
        Message::Something(circuit.to_owned()).write(&mut stream)?;

        // let msg = Message::read(&mut self.stream()?)?;
        //
        // if let Message::Something(msg) = msg {
        //     Ok(msg)
        // } else {
        //     Err(Error::new(ErrorKind::Unsupported, ""))
        // }

        Message::Close.write(&mut stream)?;
        Ok("".to_owned())
    }

    pub fn close(&self) -> io::Result<()> {
        println!("Closing connection to backend {}", self.address);
        Message::Close.write(&mut self.stream()?)?;
        Ok(())
    }

    pub fn quit(&self) -> io::Result<()> {
        println!("Quitting backend server {}", self.address);
        Message::Quit.write(&mut self.stream()?)?;
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
