use std::io::{Error, ErrorKind, Read, Result, Write};
use std::net::TcpStream;

#[derive(Debug)]
#[repr(u8)]
pub(super) enum Message {
    Quit = 0,
    Close = 1,
    Something(String) = 2,
}

impl Message {
    fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
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

    pub(super) fn read(stream: &mut TcpStream) -> Result<Self> {
        let mut discriminant = [0];

        stream.read_exact(&mut discriminant)?;

        match discriminant[0] {
            0 => Ok(Message::Quit),
            1 => Ok(Message::Close),
            2 => Ok(Message::Something(Self::read_payload(stream)?)),
            _ => Err(Error::new(ErrorKind::InvalidInput, "")),
        }
    }

    pub(super) fn write(&self, stream: &mut TcpStream) -> Result<()> {
        let mut data = match &self {
            Self::Something(payload) => add_length(payload.bytes().collect())?,
            _ => {
                vec![]
            }
        };
        data.insert(0, self.discriminant());
        dbg!(&data);
        stream.write(&data)?;
        Ok(())
    }
}

fn add_length(mut payload: Vec<u8>) -> Result<Vec<u8>> {
    let length = u32::try_from(payload.len())
        .map_err(|_| Error::new(ErrorKind::InvalidInput, ""))?
        .to_be_bytes();
    let mut data = length.to_vec();
    data.append(&mut payload);
    Ok(data)
}
