use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
pub enum More {
    TOFFOLI,
}

impl More {
    pub(super) fn elements(&self) -> usize {
        match *self {
            Self::TOFFOLI => 3,
        }
    }

    pub(super) fn targets(&self) -> usize {
        1
    }
}

impl Display for More {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::TOFFOLI => "X",
        })
    }
}
