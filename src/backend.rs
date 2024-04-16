use std::io;
use std::process::Command;

const PREFIX: &str = "qibo-backend";

pub struct Backend {
    port: usize,
}

impl Backend {
    pub fn spawn(name: &str) -> io::Result<Self> {
        let executable = format!("{PREFIX}-{name}");

        let port = 0;

        Command::new(executable).arg(port.to_string()).spawn()?;

        Ok(Self { port })
    }

    pub fn connect(port: usize) -> Self {
        Self { port }
    }
}
