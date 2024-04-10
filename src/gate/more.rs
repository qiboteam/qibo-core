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
}
