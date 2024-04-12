fn might_print(option: Option<&str>) {
    match option {
        Some(text) => println!("The argument contains the following value: '{}'", text),
        None => println!("The argument contains None."),
    }
}

pub fn main() {
    let something: Option<&str> = Some("some str");
    let nothing: Option<&str> = None;
    might_print(something);
    might_print(nothing);
}

/*
export FILE_NAME=06_option_example_str.rs
git commit -a -m "add 06_option_example_str.rs before housekeeping"
git commit -a -m "add BEFORE housekeeping => 06_option_example_str.rs "
cargo clippy --fix
cargo fmt -- --emit=files
git commit -a -m "add AFTER housekeeping => 06_option_example_str.rs"
git push
cargo run --example 06_option_example_str
*/
