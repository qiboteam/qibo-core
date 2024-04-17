use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::process::Command;

use super::address::Address;

const PREFIX: &str = "qibo-backend";

#[derive(Debug)]
pub struct Client {
    address: Address,
}

impl Client {
    /// Spawn a new server.
    pub fn spawn(name: &str) -> io::Result<Self> {
        use std::{thread, time};

        let executable = format!("{PREFIX}-{name}");

        let address = Address::new().map_err(|_| io::Error::from(io::ErrorKind::Other))?;
        println!("addr: {address}");
        println!("exec: {executable}");

        Command::new(executable).arg(&address.to_string()).spawn()?;
        // TODO: drop the sleep
        thread::sleep(time::Duration::from_millis(100));

        Ok(Self { address })
    }

    /// Connect to existing server.
    pub fn connect(address: Address) -> Self {
        Self { address }
    }

    fn stream(&self) -> io::Result<TcpStream> {
        TcpStream::connect(&self.address.to_string())
        // TcpStream::connect(&"localhost:11000")
    }

    fn write(&self, data: &str) -> io::Result<usize> {
        let bytes: Vec<_> = data.bytes().collect();
        self.stream()?.write(&bytes)
    }

    fn read(&self) -> io::Result<Vec<u8>> {
        let mut stream = self.stream()?;

        let mut buffer = [0; 1024];
        let mut message = Vec::new();

        let mut len = [0; 8];
        stream.read_exact(&mut len)?;
        let len = u64::from_le_bytes(len);
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

    pub fn execute(&self, circuit: &str) -> io::Result<String> {
        self.write(circuit)?;
        let buffer = self.read()?;
        println!("greater");

        Ok(std::str::from_utf8(&buffer).unwrap().to_owned())
    }

    pub fn close(&self) -> io::Result<()> {
        println!("Closing {} backend", self.address);
        self.write("close")?;
        Ok(())
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if let Err(_) = self.close() {
            println!("Failed closing backend {}", self.address);
        }
    }
}
