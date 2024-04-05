use super::GateT;

#[derive(Debug)]
pub struct H {}

impl GateT for H {}

#[derive(Debug)]
pub struct X {}

impl GateT for X {}

#[derive(Debug)]
pub struct Y {}

impl GateT for Y {}

#[derive(Debug)]
pub struct RX {
    pub theta: f64,
}

impl GateT for RX {}
