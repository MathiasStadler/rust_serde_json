# FROM HERE

- [FROM HERE](http://saidvandeklundert.net/learn/2021-09-01-rust-option-and-result/)
- [FROM HERE](https://www.sheshbabu.com/posts/rust-error-handling/)
- - [GITHUB REPO](https://github.com/sheshbabu/rust-error-handling-examples/tree/master)
- [good tutorial of rust basic](http://saidvandeklundert.net/learn/)

## git editor set of vi

```bash
git config --global core.editor vi
```

## git status

```bash
git status
```

## template bash script

```bash
cat << EoF > ./examples/<filename>.rs



// cargo run --example <filename>
Eof
```

## simple enum in Rust

```bash
export EXAMPLE_SCRIPT_FILE="01_simple_enums.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
fn main(){
    #[derive(Debug)]
    enum Example {
    This,
    That,
}
let _this = Example::This;
let _that = Example::That;

println!("this => {:?}",_this);
println!("that => {:?}",_that);
}

/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo clippy --fix
cargo clippy --fix --examples
cargo fmt -- --emit=files
git commit --all --message="add AFTER housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo run --example \$(echo \$FILE_NAME | cut -d . -f 1)
*/
EoF
```

## match enum in RUST

```bash
export EXAMPLE_SCRIPT_FILE="02_match_enums.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
#[derive(Debug)]
    enum Example {
    This,
    That,
    }


fn matcher(x: Example) {
    match x {
        Example::This => println!("We got This."),
        Example::That => println!("We got That."),
    }
}

fn main(){

let _this = Example::This;
let _that = Example::That;

println!("Example::This contains: {:?}", _this);
println!("Example::That contains: {:?}", _that);

matcher(Example::This);
matcher(Example::That);

}
/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo clippy --fix
cargo clippy --fix --examples
cargo fmt -- --emit=files
git commit --all --message="add AFTER housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo run --example \$(echo \$FILE_NAME | cut -d . -f 1)
*/
EoF
```

## The Option is generic over type T

```bash
cat << EoF > ./examples/03_option_generic_type.rs
#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn main(){

let a_str: Option<&str> = Some("a str");
let a_string: Option<String> = Some(String::from("a String"));
let a_float: Option<f64> = Some(1.1);
let a_vec: Option<Vec<i32>> = Some(vec![0, 1, 2, 3]);

let marie = Person {
    name: String::from("Marie"),
    age: 2,
};

let a_person: Option<Person> = Some(marie);
let maybe_someone: Option<Person> = None;

println!(
    "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n{:?}",
    a_str, a_string, a_float, a_vec, a_person, maybe_someone
);
}

// cargo run --example 03_option_generic_type

EoF
```

## Matching on the Option

```bash
cat << EoF > ./examples/04_matching_on_option.rs
pub fn main() {
let something: Option<&str> = Some("a String"); // Some("a String")
let nothing: Option<&str> = None;   // None

match something {
    Some(text) => println!("We go something: {}", text),
    None => println!("We got nothing."),
}

match nothing {
    Some(something_else) => println!("We go something: {}", something_else),
    None => println!("We got nothing"),
}
}

// cargo fmt -- --emit=files ./examples/04_matching_on_option.rs
// cargo run --example 04_matching_on_option

EoF
```

## Unwrapping the Option

```rust
cat << EoF > ./examples/05_option_unwrap.rs
/*
pub const fn unwrap(self) -> T {
    match self {
        Some(val) => val,
        None => panic!("called \`Option::unwrap()\` on a \`None\` value"),
    }
}

fn main(){

let something: Option<&str> = Some("Something");
let unwrapped = something.unwrap();
println!("{:?}\n{:?}", something, unwrapped);
let nothing: Option<&str> = None;
nothing.unwrap();

}

// cargo fmt -- --emit=files ./examples/05_option_unwrap.rs
// cargo run --example 05_option_unwrap
*/

fn main(){
    // nothing
}
EoF
```

## option-and-result -- complete sample

```rust
cat << EoF > ./examples/option_unwrap_two.rs
// FROM HERE
// https://github.com/saidvandeklundert/LearningRust/blob/master/blog/option-and-result/src/main.rs


fn main(){
let something: Option<&str> = Some("Something");
        let unwrapped = something.unwrap(); // "Something"
        println!("{:?}\n{:?}", something, unwrapped);
        let _nothing: Option<&str> = None;
        // uncommenting the next line will cause a panic:
        //nothing.unwrap();

}
// cargo run --example option_unwrap_two

EoF
```

## Passing an optional value to a function

```rust
cat << EoF > ./examples/06_option_example_str.rs
fn might_print(option: Option<&str>) {
    match option {
        Some(text) => println!("The argument contains the following value: '{}'", text),
        None => println!("The argument contains None."),
    }
}

pub fn main(){

let something: Option<&str> = Some("some str");
let nothing: Option<&str> = None;
might_print(something);
might_print(nothing);
}

/*
export FILE_NAME=06_option_example_str.rs
git commit -a -m "add BEFORE housekeeping => $FILE_NAME "
cargo clippy --fix
cargo fmt -- --emit=files
git commit -a -m "add AFTER housekeeping => $FILE_NAME"
git push
cargo run --example $(echo $FILE_NAME | cut -d . -f 1)
*/
EoF
```

## Having a function return an optional value

```rust
cat << EoF > ./examples/07_option_return_from_fn.rs
// Returns the text if it contains target character, None otherwise:
fn contains_char(text: &str, target_c: char) -> Option<&str> {
    if text.chars().any(|ch| ch == target_c) {
        return Some(text);
    } else {
        return None;
    }
}

pub fn main(){

let a = contains_char("Rust in action", 'a');
let q = contains_char("Rust in action", 'q');
println!("{:?}", a);
println!("{:?}", q);
}

/*
export FILE_NAME=07_option_return_from_fn.rs
git commit --all --message="add BEFORE housekeeping => $FILE_NAME"
git push
cargo clippy --fix
cargo clippy --fix --examples
cargo fmt -- --emit=files
git commit --all --message="add AFTER housekeeping => $FILE_NAME"
git push
cargo run --example $(echo $FILE_NAME | cut -d . -f 1)
*/
EoF
```

## three different ways to work with the Optional return - Some

```rust
cat << EoF > ./examples/08.1_option_return_work_with_three_different_ways.rs
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
let a = contains_char("Rust in action", 'a');
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
    // The third option is to capture the return of the function 
    // in a variable and use if let
    let a = contains_char("Rust in action", 'a');
    if let Some(a) = contains_char("Rust in action", 'a') {
        println!("contains_char returned something: {:?}!", a);
    }
    else {
        println!("contains_char did not return something, so branch off here")
    }
}

}

/*
export FILE_NAME="08.1_option_return_work_with_three_different_ways.rs"
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
EoF
```

## three different ways to work with the Optional return - None

```rust
cat << EoF > ./examples/09_option_return_work_with_three_different_ways_none.rs
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
println!("Some /w unwrap => contains_char returned something {:?}", a_unwrapped);

//None
// //let return_option = contains_char("Rust in action", 'x');

// called `Option::unwrap()` on a `None` value not possible
// //let a_unwrapped = a.unwrap();
// //println!("{:?}", a_unwrapped);

// unwrap() called  on a  expected None is NOT possible
// alternative used MATCH instead
//Option value => None
let _return_option = contains_char("Rust in action", 'x');

match _return_option {
    Some(contains_char) => println!("Some => Letter contains string {:?}", contains_char),
    None => println!("None => Letter NOT contains string"),
    }
}
{
// way two
// The second, safer option, is to use a match statement
let a = contains_char("Rust in action", 'a');
    match a {
        Some(a) => println!("Some => contains_char returned something: {:?}!", a),
        None => println!("None => contains_char did not return something, so branch off here"),
    }

    // None
    let a = contains_char("Rust in action", 'x');
        match a {
            Some(a) => println!("Some => contains_char returned something: {:?}!", a),
            None => println!("None => contains_char did not return something, so branch off here"),
        }
    }
{
       // way three
        // The third option is to capture the return of
        // the function in a variable and use if let
        if let Some(a) = contains_char("Rust in action", 'a') {
            println!("Some => if let => contains_char returned something: {:?}!", a);
        } 
        else {
            println!("None => if let else => contains_char did not return something, so branch off here")
        }

        //None
        if let Some(a) = contains_char("Rust in action", 'x') {
            println!("Some => if let => contains_char returned something: {:?}!", a);
        } 
        else {
            println!("None => if let else => contains_char did not return something, so branch off here")
        }
    }
}

/*
export FILE_NAME="09_option_return_work_with_three_different_ways_none.rs"
export FILE_DIR_NAME="./examples"
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo clippy --fix
cargo clippy --fix --examples
cargo fmt -- --emit=files
git commit --all --message="add AFTER housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo run --example \$(echo \$FILE_NAME | cut -d . -f 1)
*/
EoF
```
