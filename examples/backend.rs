use qibo_core::prelude::*;

fn main() {
    let backend = Client::spawn("numpy").expect("Backend not found.");
    let res = backend
        .execute("ciao, come va?\nbene grazie\ne tu?")
        .unwrap();
    println!("{}", res);
}
