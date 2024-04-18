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
        Example::This => println!("We got This. {:?}",x),
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
export EXAMPLE_SCRIPT_FILE="03_option_generic_type.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
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

## Matching on the Option

```bash
export EXAMPLE_SCRIPT_FILE="04_matching_on_option.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
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

## Unwrapping the Option

```rust
export EXAMPLE_SCRIPT_FILE="05_option_unwrap.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
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

## option-and-result -- complete sample

```rust
export EXAMPLE_SCRIPT_FILE="05_option_unwrap_two.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
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

## Passing an optional value to a function

```rust
export EXAMPLE_SCRIPT_FILE="06_option_example_str.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
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
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo clippy --fix
cargo clippy --fix --examples
cargo check --verbose
cargo check --verbose --examples 
cargo fmt -- --emit=files
git commit --all --message="add AFTER housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo run --example \$(echo \$FILE_NAME | cut -d . -f 1)
*/
EoF
```

## Having a function return an optional value

```rust
export EXAMPLE_SCRIPT_FILE="07_option_return_from_fn.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
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
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo clippy --fix
cargo clippy --fix --examples
// cargo check --verbose
// cargo check --verbose --examples 
cargo check 
cargo check --examples 
cargo fmt -- --emit=files
git commit --all --message="add AFTER housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo run --example \$(echo \$FILE_NAME | cut -d . -f 1)
*/
EoF
```

## three different ways to work with the Optional return - Some

```rust
export EXAMPLE_SCRIPT_FILE="08_option_return_work_with_three_different_ways.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
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
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo clippy --fix
cargo clippy --fix --examples
// cargo check --verbose
// cargo check --verbose --examples
cargo check 
cargo check --examples 
cargo fmt -- --emit=files
git commit --all --message="add AFTER housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo run --example \$(echo \$FILE_NAME | cut -d . -f 1)
*/
EoF
```

## three different ways to work with the Optional return - None

```rust
export EXAMPLE_SCRIPT_FILE="09_option_return_work_with_three_different_ways_none.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
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
        None => println!("unwrap None not possible instead => None => Letter NOT contains string"),
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
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo install --list
cargo update --workspace 
cargo clippy --fix
cargo clippy --fix --examples
// cargo check --verbose
// cargo check --verbose --examples
cargo check 
cargo check --examples 
cargo fmt -- --emit=files
git commit --all --message="add AFTER housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo run --example \$(echo \$FILE_NAME | cut -d . -f 1)
*/
EoF
```

## continue here => [Optional values inside a struct](http://saidvandeklundert.net/learn/2021-09-01-rust-option-and-result/)

## Optional values inside a struct

- We can also use the Option inside a struct. This might be useful in case a field may or may not have any value

```rust
export EXAMPLE_SCRIPT_FILE="10_option_inside_struct.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    age: Option<i32>,
}

fn main(){

    let marie = Person {
        name: String::from("Marie"),
        age: Some(2),
    };

    let jan = Person {
        name: String::from("Jan"),
        age: None,
    };
    println!("{:?}\n{:?}", marie, jan);
}

/*
export FILE_NAME=$EXAMPLE_SCRIPT_FILE
export FILE_DIR_NAME=$EXAMPLE_SCRIPT_DIR
git add \$FILE_DIR_NAME/\$FILE_NAME
git commit --all --message="-> Add BEFORE housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
# cargo install --list
cargo update --workspace 
cargo clippy --fix
cargo clippy --fix --examples
// cargo check --verbose
// cargo check --verbose --examples 
cargo check
cargo check --examples
cargo fmt -- --emit=files
git commit --all --message="-> Add AFTER housekeeping => \$FILE_DIR_NAME/\$FILE_NAME"
git push
cargo run --example \$(echo \$FILE_NAME | cut -d . -f 1)
*/
EoF
```

## Real world example /w vec->pop

- An example where the Option is used inside Rust is the pop method for vectors
- - This method return an generic Option

```rust
export EXAMPLE_SCRIPT_FILE="11_option_inside_struct_real_example_pop.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    age: Option<i32>,
}

fn main(){

    let marie = Person {
        name: String::from("Marie"),
        age: Some(2),
    };

    let jan = Person {
        name: String::from("Jan"),
        age: None,
    };
    println!("{:?}\n{:?}", marie, jan);
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
*/
EoF
```

## The result

- Another important construct in Rust is the __Result__ enum. Same as with the Option, the Result is an enum

```rust
pub enum Result<T, E> {
    /// Contains the success value
    Ok(T),
    /// Contains the error value    
    Err(E),
}
```

- The Result enum is generic over 2 types, given the name T and E.
-- The T is used for the OK variant, which is used to express a successful result.
-- The E is used for the Err variant, used to express an error value

## Matching on the Result

```rust
export EXAMPLE_SCRIPT_FILE="12_dbg_on_the_result.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE

fn check_length(s: &str, min: usize) -> Result<&str, String> {
    if s.chars().count() >= min {
        return Ok(s)
    } else {
        return Err(format!("'{}' is not long enough!", s))
    }
}

fn main(){
    let _a = check_length("some str", 5);

    let _b = check_length("another str", 300);

    // generate warning: unused  that must be used dbg!
    // marker with underline => generate NOT warning

    _ = dbg!(_a); // Ok("some str",)
    _ = dbg!(_b); // Err("'another str' is not long enough!",)
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
*/
EoF
```

## We can use a match expression to deal with the Result

```rust
export EXAMPLE_SCRIPT_FILE="13_matching_on_the_result.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE

fn check_length(s: &str, min: usize) -> Result<&str, String> {
    if s.chars().count() >= min {
        return Ok(s)
    } else {
        return Err(format!("'{}' is not long enough!", s))
    }
}

fn main(){
    // let a = check_length("some str", 5);
    // let b = check_length("another str", 300);
    // dbg!(a); // Ok("some str",)
    // dbg!(b); // Err("'another str' is not long enough!",)
    
    // instead /w match
    //Ok 
    let func_return = check_length("some str", 5);
    let a_str = match func_return {
    Ok(a_str) => a_str,
    Err(error) => panic!("Err => Problem running 'check_length':\n {:?}", error),
        };
    // after check_length
    println!("Length is Ok -> this str is long enough! => {} <=",a_str);
    println!("Can use the variable => {} ",a_str);

    // instead /w match
    // Err
    let func_return = check_length("some string literal", 100);
    #[allow(unused_variables)]
    let _a_str = match func_return {
    Ok(a_str) => a_str,
    Err(error) => panic!("Err => Problem running 'check_length':\n {:?}", error),
};
// thread 'main' panicked at 'Problem running 'check_length':
// "'some string literal' is not long enough!"'

}//end of fn main

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
*/
EoF
```

## continue here =>  [Unwrapping the Result](http://saidvandeklundert.net/learn/2021-09-01-rust-option-and-result/)

## Unwrapping the Result

- Instead of using a match expression, there is also a shortcut that you’ll come across very often. This shortcut is the unwrap method that is defined for the Result enum

```rust
impl<T, E: fmt::Debug> Result<T, E> {
    ...
    pub fn unwrap(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", &e),
        }
    }
    ...
}

```

## example with use unwrap

```rust
export EXAMPLE_SCRIPT_FILE="14_result_unwrap.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE

fn main(){
   let json_string = r#"
{
    "key": "value",
    "another key": "another value",
    "key to a list" : [1 ,2]
}"#;

let invalid_json_string = r#"
{
    // The Error is ::
    "key":: "value",
    "another key": "another value",
    "key to a list" : [1 ,2]
}"#;

let json_serialized: serde_json::Value = serde_json::from_str(&json_string).unwrap();
println!("Ok => {:?}", &json_serialized);
// Object({"another key": String("another value"), "key": String("value"), "key to a list": Array([Number(1), Number(2)])})

let invalid_json_serialized: serde_json::Value = serde_json::from_str(&invalid_json_string).unwrap();
println!("Err => {:?}", &invalid_json_serialized);
//called `Result::unwrap()` on an `Err` value: Error("expected value", line: 4, column: 19)

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
*/
EoF
```

## example with use unwrap error handling with expect

```rust
export EXAMPLE_SCRIPT_FILE="15_result_unwrap_error_handling_with_expect.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE

fn main(){
   let json_string = r#"
{
    "key": "value",
    "another key": "another value",
    "key to a list" : [1 ,2]
}"#;

let invalid_json_string = r#"
{
    // The Error is 
    "key" "value",
    "another key": "another value",
    "key to a list" : [1 ,2]
}"#;

// let json_serialized: serde_json::Value = serde_json::from_str(&json_string).unwrap();
// instead /w .expect("unable to deserialize JSON");
let json_serialized: serde_json::Value = serde_json::from_str(&json_string).expect("MESSAGE EXPECT => unable to deserialize JSON");


println!("Ok => {:?}", &json_serialized);
// Object({"another key": String("another value"), "key": String("value"), "key to a list": Array([Number(1), Number(2)])})

// let invalid_json_serialized: serde_json::Value = serde_json::from_str(&invalid_json_string).unwrap();
// instead /w .expect("unable to deserialize JSON");
let invalid_json_serialized: serde_json::Value = serde_json::from_str(&invalid_json_string).expect("MESSAGE EXPECT => unable to deserialize JSON");
// Is NOT executed because the call of the macro panic!
println!("Err => {:?}", &invalid_json_serialized);
//called `Result::unwrap()` on an `Err` value: Error("expected value", line: 4, column: 19)

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
*/
EoF
```

## continue here => [Using ? and handling different errors](http://saidvandeklundert.net/learn/2021-09-01-rust-option-and-result/)

## generate json test file

```rust
export EXAMPLE_SCRIPT_FILE="16_generate_json_file.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE
// FROM HERE
// https://stackoverflow.com/questions/73505520/how-to-write-a-string-to-file

use std::fs::OpenOptions;
 use std::io::Write;
fn main() {

let json_string = r#"{
        "name": "MyName",
        "age": 69
    }
"#;
    
    // create file
    let mut test_file = OpenOptions::new()
        .truncate(true)
        // .create_new(true)
        .create(true)
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
*/
EoF
```

## Using ? and handling different errors

```rust
export EXAMPLE_SCRIPT_FILE="17_using_question_mark_and_handling_different_errors.rs"
export EXAMPLE_SCRIPT_DIR="examples"
cat << EoF > ./$EXAMPLE_SCRIPT_DIR/$EXAMPLE_SCRIPT_FILE

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

// Debug allows us to print the struct.
// Deserialize and Serialize adds decoder and encoder capabilities to the struct.
#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
    age: usize,
}

fn file_to_json(s: &str) -> Result<Person, Box<dyn Error>> {
    let text = fs::read_to_string(s)?;
    let marie: Person = serde_json::from_str(&text)?;
    Ok(marie)
}


fn main(){

    // _ <- underline avoid err
    // warning: unused  that must be used dbg!
    // marker with underline
    let _x = file_to_json("/tmp/json.txt");
    let _y = file_to_json("/tmp/invalid_json.txt");
    let _z = file_to_json("/tmp/non_existing_file.txt");

    _ = dbg!(_x);
    _ = dbg!(_y);
    _ = dbg!(_z);
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
*/
EoF
```
