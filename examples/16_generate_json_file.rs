// FROM HERE
// https://stackoverflow.com/questions/73505520/how-to-write-a-string-to-file

use std::fs::OpenOptions;
use std::io::Write;
fn main() {
    let json_string = r#"{
        "name": "MyName",
        "age": "69"
    }
"#;

    // create file
    let mut test_file = OpenOptions::new()
        .truncate(true)
        // .create_new(true)
        .read(true)
        .write(true)
        .open("/tmp/json.txt")
        .unwrap();

    //  write json sting to file
    let eg = test_file.write_all(json_string.as_bytes());
    match eg {
        Ok(()) => println!("OK"),
        Err(e) => println!("{:?}", e),
    }
}

/*
export FILE_NAME=16_generate_json_file.rs
export FILE_DIR_NAME=examples
git add $FILE_DIR_NAME/$FILE_NAME
git commit --all --message="-> Add BEFORE housekeeping => $FILE_DIR_NAME/$FILE_NAME"
git push
# cargo install --list
# cargo update --workspace
cargo clippy --fix
cargo clippy --fix --examples
# cargo check --verbose
# cargo check --verbose --examples
cargo check
cargo check --examples
cargo fmt -- --emit=files
git commit --all --message="-> Add AFTER housekeeping => $FILE_DIR_NAME/$FILE_NAME"
git push
cargo run --example $(echo $FILE_NAME | cut -d . -f 1)
*/
