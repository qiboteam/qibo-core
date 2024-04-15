use std::fmt::{self, Display};

use super::extract_name;

#[cfg_attr(doc, katexit::katexit)]
#[derive(Debug, Clone, Copy)]
pub enum One {
    /// The Hadamard gate.
    ///
    /// $$
    /// \\frac{1}{\\sqrt{2}} \\, \\begin{pmatrix}
    /// 1 & 1 \\\\
    /// 1 & -1 \\\\
    /// \\end{pmatrix}
    /// $$
    H,
    /// The Pauli-$X$ gate.
    ///
    /// $$
    /// \\begin{pmatrix}
    /// 0 & 1 \\\\
    /// 1 & 0 \\\\
    /// \\end{pmatrix}
    /// $$
    X,
    /// The Pauli-$Y$ gate.
    ///
    /// $$
    /// \\begin{pmatrix}
    /// 0 & -i \\\\
    /// i & 0 \\\\
    /// \\end{pmatrix}
    /// $$
    Y,
    /// The Pauli-$Z$ gate.
    ///
    /// $$
    /// \\begin{pmatrix}
    /// 1 & 0 \\\\
    /// 0 & -1 \\\\
    /// \\end{pmatrix}
    /// $$
    Z,
    /// The $\sqrt{X}$ gate.
    ///
    /// $$
    /// \\frac{1}{2} \\, \\begin{pmatrix}
    /// 1 + i & 1 - i \\\\
    /// 1 - i & 1 + i \\\\
    /// \\end{pmatrix}
    /// $$
    SX,
    /// The conjugate transpose of the $\sqrt{X}$ gate.
    ///
    /// $$
    /// \\frac{1}{2} \\, \\begin{pmatrix}
    /// 1 - i & 1 + i \\\\
    /// 1 + i & 1 - i \\\\
    /// \\end{pmatrix}
    /// $$
    SXDG,
    /// The $S$ gate.
    ///
    /// $$
    /// \\begin{pmatrix}
    /// 1 & 0 \\\\
    /// 0 & i \\\\
    /// \\end{pmatrix}
    /// $$
    S,
    RX(f64),
}

impl Display for One {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&match self {
            g @ (Self::H
            | Self::X
            | Self::Y
            | Self::Z
            | Self::SX
            | Self::SXDG
            | Self::S
            | Self::RX(_)) => extract_name(g),
        })
    }
}
