pub use self::more::More;
pub use self::one::One;
pub use self::two::Two;

mod more;
mod one;
mod two;

#[derive(Debug, Clone, Copy)]
pub enum Gate {
    One(One),
    Two(Two),
    More(More),
}
