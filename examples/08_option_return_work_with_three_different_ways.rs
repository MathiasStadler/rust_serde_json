// Returns the text if it contains target character, None otherwise:
fn contains_char(text: &str, target_c: char) -> Option<&str> {
    if text.chars().any(|ch| ch == target_c) {
        return Some(text);
    } else {
        return None;
    }
}

pub fn main() {
    // way one
    let a = contains_char("Rust in action", 'a');
    let a_unwrapped = a.unwrap();
    println!("{:?}", a_unwrapped);
}

/*
export FILE_NAME=08_option_return_work_with_three_different_ways.rs
git commit --all --message="add BEFORE housekeeping => 07_option_return_from_fn.rs"
git push
cargo clippy --fix
cargo clippy --fix --examples
cargo fmt -- --emit=files
git commit --all --message="add AFTER housekeeping => 07_option_return_from_fn.rs"
git push
cargo run --example 07_option_return_from_fn
*/
