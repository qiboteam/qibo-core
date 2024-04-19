use std::io::{Error, ErrorKind, Read, Result, Write};
use std::net::TcpStream;

fn discriminant<E>(var: &E) -> u8 {
    unsafe { *(var as *const E as *const u8) }
}

fn read_discriminant(stream: &mut TcpStream) -> Result<u8> {
    let mut discriminant = [0];
    stream.read_exact(&mut discriminant)?;
    Ok(discriminant[0])
}

fn read_payload(stream: &mut TcpStream) -> Result<String> {
    // TODO: this limits the size of the message to the usual 4GB
    // it should not be a limitation, since larger messages should be exchanged through
    // different channels - but it has to be documented
    let mut len = [0; 4];
    stream.read_exact(&mut len)?;
    let len: usize = u32::from_be_bytes(len)
        .try_into()
        .map_err(|_| Error::new(ErrorKind::InvalidInput, ""))?;

    let mut message = Vec::with_capacity(len);
    message.resize(len, 0);
    stream.read_exact(&mut message)?;

    String::from_utf8(message).map_err(|_| Error::new(ErrorKind::InvalidInput, ""))
}

fn write_length(mut payload: Vec<u8>) -> Result<Vec<u8>> {
    let length = u32::try_from(payload.len())
        .map_err(|_| Error::new(ErrorKind::InvalidInput, ""))?
        .to_be_bytes();
    let mut data = length.to_vec();
    data.append(&mut payload);
    Ok(data)
}

fn write_message(stream: &mut TcpStream, discriminant: u8, mut data: Vec<u8>) -> Result<()> {
    data.insert(0, discriminant);
    stream.write(&data)?;
    Ok(())
}

#[derive(Debug)]
#[repr(u8)]
pub(super) enum FromClient {
    Quit = 0,
    Subscribe = 1,
    Close = 2,
    Something(String) = 3,
}

impl FromClient {
    pub(super) fn read(stream: &mut TcpStream) -> Result<Self> {
        match read_discriminant(stream)? {
            0 => Ok(Self::Quit),
            1 => Ok(Self::Subscribe),
            2 => Ok(Self::Close),
            3 => Ok(Self::Something(read_payload(stream)?)),
            _ => Err(Error::new(ErrorKind::InvalidInput, "")),
        }
    }

    pub(super) fn write(&self, stream: &mut TcpStream) -> Result<()> {
        let data = match &self {
            Self::Something(payload) => write_length(payload.bytes().collect())?,
            _ => {
                vec![]
            }
        };
        write_message(stream, discriminant(self), data)
    }
}

#[derive(Debug)]
#[repr(u8)]
pub(super) enum FromServer {
    Reply(String) = 0,
}

impl FromServer {
    pub(super) fn read(stream: &mut TcpStream) -> Result<Self> {
        match read_discriminant(stream)? {
            0 => Ok(Self::Reply(read_payload(stream)?)),
            _ => Err(Error::new(ErrorKind::InvalidInput, "")),
        }
    }

    pub(super) fn write(&self, stream: &mut TcpStream) -> Result<()> {
        let data = match &self {
            Self::Reply(payload) => write_length(payload.bytes().collect())?,
        };
        write_message(stream, discriminant(self), data)
    }
}
