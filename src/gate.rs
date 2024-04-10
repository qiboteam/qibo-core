pub use self::more::More::{self, *};
pub use self::one::One::{self, *};
pub use self::two::Two::{self, *};

mod more;
mod one;
mod two;

#[derive(Debug, Clone, Copy)]
pub enum Gate {
    One(One),
    Two(Two),
    More(More),
}

impl From<One> for Gate {
    fn from(gate: One) -> Self {
        Self::One(gate)
    }
}

impl From<Two> for Gate {
    fn from(gate: Two) -> Self {
        Self::Two(gate)
    }
}

impl From<More> for Gate {
    fn from(gate: More) -> Self {
        Self::More(gate)
    }
}

impl Gate {
    pub(crate) fn elements(&self) -> usize {
        match *self {
            Self::One(_) => 1,
            Self::Two(_) => 2,
            Self::More(gate) => gate.elements(),
        }
    }
}
