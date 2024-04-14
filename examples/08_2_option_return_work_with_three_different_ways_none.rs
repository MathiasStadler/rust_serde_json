// Returns the text if it contains target character, None otherwise:
fn contains_char(text: &str, target_c: char) -> Option<&str> {
    if text.chars().any(|ch| ch == target_c) {
        return Some(text);
    } else {
        return None;
    }
}

pub fn main(){

{
// way one
// The first one, which is the least safe, would be simply calling unwrap
//Some
let a = contains_char("Rust in action", 'a');
let a_unwrapped = a.unwrap();
println!("{:?}", a_unwrapped);
//None
let return_option = contains_char("Rust in action", 'x');



// called  on a  value not possible
let a_unwrapped = a.unwrap();
println!("{:?}", a_unwrapped);
}

{
// way two
// The second, safer option, is to use a match statement
let a = contains_char("Rust in action", 'a');
match a {
    Some(a) => println!("contains_char returned something: {:?}!", a),
    None => println!("contains_char did not return something, so branch off here"),
}

}
{
    // way three
    // The third option is to capture the return of 
    // the function in a variable and use if let
    let a = contains_char("Rust in action", 'a');
    if let Some(a) = contains_char("Rust in action", 'a') {
    println!("contains_char returned something: {:?}!", a);
    } else {
    println!("contains_char did not return something, so branch off here")
    }

}

}

/*
export FILE_NAME="08_2_option_return_work_with_three_different_ways_none.rs"
export FILE_DIR_NAME="./examples"
git add $FILE_DIR_NAME/$FILE_NAME
git commit --all --message="add BEFORE housekeeping => $FILE_DIR_NAME/$FILE_NAME"
git push
cargo clippy --fix
cargo clippy --fix --examples
cargo fmt -- --emit=files
git commit --all --message="add AFTER housekeeping => $FILE_DIR_NAME/$FILE_NAME"
git push
cargo run --example $(echo $FILE_NAME | cut -d . -f 1)
*/