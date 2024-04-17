use qibo_core::prelude::*;

fn main() {
    let backend = Backend::spawn("numpy").expect("Backend not found.");
    let res = backend.execute("42").unwrap();
    println!("{}", res);
    backend.close().unwrap();
}
