use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;

const PREFIX: &str = "qibo-backend";
const LOCAL: &str = "127.0.0.1";

#[derive(Debug)]
struct Port(u16);

impl Port {
    fn available(&self) -> bool {
        TcpListener::bind((LOCAL, self.0)).is_ok()
    }

    fn new() -> Result<Self, ()> {
        (10000..11000)
            .find(|p| Self(*p).available())
            .map(|p| Self(p))
            .ok_or(())
    }
}

impl From<u16> for Port {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct Backend {
    port: Port,
}

impl Backend {
    pub fn spawn(name: &str) -> io::Result<Self> {
        let executable = format!("{PREFIX}-{name}");

        let port = Port::new().map_err(|_| io::Error::from(io::ErrorKind::Other))?;

        Command::new(executable).arg(port.0.to_string()).spawn()?;

        Ok(Self { port })
    }

    pub fn connect(port: u16) -> Self {
        Self { port: port.into() }
    }

    fn address(&self) -> String {
        let port = self.port.0;
        format!("{LOCAL}:{port}")
    }

    pub fn execute(&self, a: u8) -> io::Result<String> {
        let mut stream = TcpStream::connect(self.address())?;
        let mut buffer = [0; 10];

        stream.write(&[a])?;
        stream.read(&mut buffer[..])?;

        Ok(std::str::from_utf8(&buffer).unwrap().to_owned())
    }
}
