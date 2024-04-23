use std::io::Result;

use pyo3::prelude::*;
use qibo_core::prelude;

use crate::circuit::circuit::Circuit;

#[pymodule]
pub mod backend {
    use super::*;

    #[pyclass]
    #[derive(Clone)]
    struct Address(prelude::Address);

    #[pyclass]
    struct Client(prelude::Client);

    #[pymethods]
    impl Client {
        #[staticmethod]
        fn spawn(name: &str) -> Result<Self> {
            Ok(Self(prelude::Client::spawn(name)?))
        }

        #[staticmethod]
        fn connect(address: Address) -> Result<Self> {
            Ok(Self(prelude::Client::connect(address.0)?))
        }

        pub fn execute(&mut self, circuit: &Circuit) -> Result<String> {
            self.0.execute(&circuit.0)
        }

        pub fn close(&mut self) -> Result<()> {
            self.0.close()
        }

        pub fn quit(&mut self) -> Result<()> {
            self.0.quit()
        }
    }
}
