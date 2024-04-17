fn check_length(s: &str, min: usize) -> Result<&str, String> {
    if s.chars().count() >= min {
        Ok(s)
    } else {
        Err(format!("'{}' is not long enough!", s))
    }
}

fn main() {
    let a = check_length("some str", 5);
    let b = check_length("another str", 300);
    #[allow(unused_must_use)]
    dbg!(a); // Ok("some str",)
    #[allow(unused_must_use)]
    dbg!(b); // Err("'another str' is not long enough!",)
}

/*
export FILE_NAME=12_dbg_on_the_result.rs
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
