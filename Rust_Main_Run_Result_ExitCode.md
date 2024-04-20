# Rust_main_run_result_exitcode

## add exitcode to project

```bash
cargo add exitcode
```

## main exitcode hello world

```rust
export EXAMPLE_SCRIPT_FILE="20_exitcode_hello_world.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
// FROM HERE
// https://rust-cli.github.io/book/in-depth/exit-code.html


fn main(){

    println!("Hello World");
    println!("see output of =>  echo $?");
    std::process::exit(exitcode::OK);
}

/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="-> Add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
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
git commit --all --message="-> Add AFTER housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo run --example \$(echo \$FILE_NAME | cut -d . -f 1)
echo "ReturnCode => \$?"
*/
EoF
```

## eprintln! => print error message

```rust
export EXAMPLE_SCRIPT_FILE="21_eprintln.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
// FROM HERE
// https://github.com/rust-lang/rust/issues/113081
// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

fn real_main() -> ExitCode {
    // ... do something ...
    ExitCode::FAILED
}

fn main() -> ExitCode {
    let result_exit_code = real_main();

    let exit_code match result_exit_code {
        Ok(exit_code) => println("Ok Exit Code {:?}",exit_code),
        Err(error) => eprintln("Err Exit Code {:?}",error),
    }

    if !exit_code.is_success() {
        eprintln!("FAILED!");
    }


    println!("see output of =>  echo $?");
    std::process::exit(exitcode::OK);
}

/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="-> Add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
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
git commit --all --message="-> Add AFTER housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo run --example \$(echo \$FILE_NAME | cut -d . -f 1)
echo "ReturnCode => \$?"
*/
EoF
```

## rust script template

```rust
export EXAMPLE_SCRIPT_FILE="99_template.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE

fn main(){

    println!("template");
}

/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="-> Add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
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
git commit --all --message="-> Add AFTER housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo run --example \$(echo \$FILE_NAME | cut -d . -f 1)
echo $?
*/
EoF
```

## [to highlight a "Note" and "Warning" using blockquote](https://github.com/orgs/community/discussions/16925)

- note

> [!NOTE]  
> Highlights information that users should take into account, even when skimming.

- tip

> [!TIP]
> Optional information to help a user be more successful.

- important

> [!IMPORTANT]  
> Crucial information necessary for users to succeed.

- warning

> [!WARNING]  
> Critical content demanding immediate user attention due to potential risks.

- caution

> [!CAUTION]
> Negative potential consequences of an action.
