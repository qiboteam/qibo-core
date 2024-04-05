use enum_dispatch::enum_dispatch;

use self::single::{H, RX, X, Y};

pub(crate) mod single;

#[enum_dispatch]
trait GateT {}

#[enum_dispatch(GateT)]
#[derive(Debug)]
pub enum Gate {
    H,
    X,
    Y,
    RX,
}
