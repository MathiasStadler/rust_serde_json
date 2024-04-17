// use serde_json::json;

fn main(){
   let json_string = r#"
{
    "key": "value",
    "another key": "another value",
    "key to a list" : [1 ,2]
}"#;
let json_serialized: serde_json::Value = serde_json::from_str(&json_string).unwrap();
println!("{:?}", &json_serialized);
// Object({"another key": String("another value"), "key": String("value"), "key to a list": Array([Number(1), Number(2)])})

}

/*
export FILE_NAME=14_result_unwrap.rs
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
