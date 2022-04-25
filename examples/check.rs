
use my_serde::MySerde;

#[derive(MySerde)]
struct B {
    b: i32,
    c: usize,
    d: u64
}

fn main() {
    let b = B {
        b: 5, c: 6, d: 7
    };
    b.serialize();
}