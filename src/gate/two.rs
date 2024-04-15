use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
pub enum Two {
    CNOT,
    CU1(f64),
    SWAP,
}

impl Display for Two {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::CNOT => "X",
            Self::CU1(_) => "U1",
            Self::SWAP => "x",
        })
    }
}
