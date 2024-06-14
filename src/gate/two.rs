use std::fmt::{self, Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Two {
    CNOT,
    CU1(f64),
    SWAP,
}

impl Two {
    pub(super) fn targets(&self) -> usize {
        match *self {
            Self::SWAP => 2,
            _ => 1,
        }
    }
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
