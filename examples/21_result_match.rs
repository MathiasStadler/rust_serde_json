// FROM HERE
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/std/error/trait.Error.html

use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct SuperError {
    side: SuperErrorSideKick,
}

impl fmt::Display for SuperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SuperError is here!")
    }
}

impl Error for SuperError {
    fn description(&self) -> &str {
        "I'm the superhero of errors"
    }

    fn cause(&self) -> Option<&dyn Error> {
        Some(&self.side)
    }
}

#[derive(Debug)]
struct SuperErrorSideKick;

impl fmt::Display for SuperErrorSideKick {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

impl Error for SuperErrorSideKick {
    fn description(&self) -> &str {
        "I'm SuperError side kick"
    }
}

fn get_super_error() -> Result<(), SuperError> {
    Err(SuperError {
        side: SuperErrorSideKick,
    })
}

fn main() {
    match get_super_error() {
        Err(e) => {
            println!("Error: {}", e.description());
            println!("Caused by: {}", e.cause().unwrap());
        }
        _ => println!("No error"),
    }
}

/*
export FILE_NAME=21_result_match.rs
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
echo "ReturnCode => $?"
*/
