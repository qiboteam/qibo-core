use std::convert::TryFrom;
use std::fmt::{self, Display};
use std::net::TcpListener;
use std::ops::Range;

pub(super) const LOCAL: &str = "localhost";
const PORT_RANGE: Range<u16> = 10000..11000;

#[derive(Debug, Clone, Copy)]
pub struct Address(u16);

impl Address {
    fn available(&self) -> bool {
        TcpListener::bind((LOCAL, self.0)).is_ok()
    }

    pub(super) fn new() -> Result<Self, ()> {
        PORT_RANGE
            .clone()
            .find(|p| Self(*p).available())
            .map(|p| Self(p))
            .ok_or(())
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{LOCAL}:{}", self.0)
    }
}

impl TryFrom<&str> for Address {
    type Error = ();

    fn try_from(arg: &str) -> Result<Self, ()> {
        let port = arg.split(":").collect::<Vec<_>>()[1];
        Ok(Address(port.parse().map_err(|_| ())?))
    }
}
