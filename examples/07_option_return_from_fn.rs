// Returns the text if it contains target character, None otherwise:
fn contains_char(text: &str, target_c: char) -> Option<&str> {
    if text.chars().any(|ch| ch == target_c) {
        Some(text)
    } else {
        None
    }
}

pub fn main() {
    let a = contains_char("Rust in action", 'a');
    let q = contains_char("Rust in action", 'q');
    println!("{:?}", a);
    println!("{:?}", q);
}

/*
export FILE_NAME=07_option_return_from_fn.rs
git commit -a -m "add BEFORE housekeeping => 06_option_example_str.rs "
cargo clippy --fix
cargo fmt -- --emit=files
git commit -a -m "add AFTER housekeeping => 06_option_example_str.rs"
git push
cargo run --example 06_option_example_str
*/
