use std::io;
use std::net::TcpListener;
use std::process::Command;

const PREFIX: &str = "qibo-backend";

#[derive(Debug)]
struct Port(u16);

impl Port {
    fn available(&self) -> bool {
        TcpListener::bind(("127.0.0.1", self.0)).is_ok()
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
}
