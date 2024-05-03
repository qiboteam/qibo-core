use crate::circuit::Circuit;
use crate::state::State;

use ndarray::{ArrayD, IxDyn};

pub(super) fn request<T>(circuit: Circuit, state: Option<State<T>>) -> Vec<u8> {
    vec![]
}

pub(super) fn response(msg: Vec<u8>) -> State<usize> {
    State(ArrayD::<usize>::zeros(IxDyn(&[8])))
}
