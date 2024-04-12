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

// cargo run --example 06_option_example_str
