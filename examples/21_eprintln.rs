// FROM HERE
// https://github.com/rust-lang/rust/issues/113081
// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

use std::process::ExitCode;

fn real_main() -> ExitCode {
    // ... do something ...
    ExitCode::FAILURE
}

fn main() -> ExitCode {
    let result_exit_code = real_main();

    let exit_code =  match result_exit_code {
        Ok(exit_code) => println("Ok Exit Code {:?}",exit_code),
        Err(error) => eprintln("Err Exit Code {:?}",error),
    };

    if !exit_code.is_success() {
        eprintln!("FAILED!");
    }


    println!("see output of =>  echo 0");
    std::process::exit(exitcode::OK);
}

/*
export FILE_NAME=21_eprintln.rs
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
