// FROM HERE
// https://rust-cli.github.io/book/in-depth/exit-code.html

fn main() {
    println!("Hello World");
    println!("see output of =>  echo 0");
    std::process::exit(exitcode::OK);
}

/*
export FILE_NAME=20_exitcode_hello_world.rs
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
