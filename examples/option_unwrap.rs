pub const fn unwrap(self) -> T {
    match self {
        Some(val) => val,
        None => panic!("called `Option::unwrap()` on a `None` value"),
    }
}

fn main() {
    let something: Option<&str> = Some("Something");
    let unwrapped = something.unwrap();
    println!("{:?}\n{:?}", something, unwrapped);
    let nothing: Option<&str> = None;
    nothing.unwrap();
}

// cargo fmt -- --emit=files ./examples/option_unwrap.rs
// cargo run --example option_unwrap
