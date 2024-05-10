extern "C" {
    fn example();
}

fn main() {
    unsafe {
        example();
    }
}
