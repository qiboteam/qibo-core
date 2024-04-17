use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::ops::{Deref, Range};
use std::process::Command;

const PREFIX: &str = "qibo-backend";
const LOCAL: &str = "127.0.0.1";
const PORT_RANGE: Range<u16> = 10000..11000;

#[derive(Debug)]
struct Port(u16);

impl Port {
    fn available(&self) -> bool {
        TcpListener::bind((LOCAL, self.0)).is_ok()
    }

    fn new() -> Result<Self, ()> {
        PORT_RANGE
            .clone()
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

impl Deref for Port {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
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
        println!("{executable}:{}", port.0);

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

    fn stream(&self) -> io::Result<TcpStream> {
        TcpStream::connect(self.address())
    }

    fn write(&self, data: &str) -> io::Result<usize> {
        let bytes: Vec<_> = data.bytes().collect();
        self.stream()?.write(&bytes)
    }

    fn read(&self, buffer: &mut [u8]) -> io::Result<()> {
        self.stream()?.read(buffer)?;
        Ok(())
    }

    pub fn execute(&self, circuit: &str) -> io::Result<String> {
        let mut buffer = [0; 10];

        self.write(circuit)?;
        self.read(&mut buffer[..])?;

        Ok(std::str::from_utf8(&buffer).unwrap().to_owned())
    }

    pub fn close(&self) -> io::Result<()> {
        self.write("close")?;
        Ok(())
    }
}
