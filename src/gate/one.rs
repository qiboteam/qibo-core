use std::fmt::{self, Display};

use super::extract_name;

#[derive(Debug, Clone, Copy)]
pub enum One {
    H,
    X,
    Y,
    Z,
    RX(f64),
}

impl Display for One {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&match self {
            g @ (Self::H | Self::X | Self::Y | Self::Z | Self::RX(_)) => extract_name(g),
        })
    }
}
