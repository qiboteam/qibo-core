use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
pub enum Two {
    SWAP,
}

impl Two {
    pub(super) fn targets(&self) -> usize {
        match *self {
            Self::SWAP => 2,
        }
    }
}

impl Display for Two {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::SWAP => "x",
        })
    }
}
