# [FROM HERE](http://saidvandeklundert.net/learn/2021-09-01-rust-option-and-result/)

## template bash script

```bash
cat << EoF > ./examples/<filename>.rs



// cargo run --example <filename>
Eof
```

## simple enum in Rust

```bash
cat << EoF > ./examples/simple_enums.rs
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
// cargo run --example simple_enums
EoF
```

## match enum in RUST

```bash
cat << EoF > ./examples/match_enums.rs

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
// cargo run --example match_enums
EoF
```

## The Option is generic over type T

```bash
cat << EoF > ./examples/option_generic_type.rs
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

// cargo run --example option_generic_type

EoF
```

## Matching on the Option

```bash
cat << EoF > ./examples/matching_on_option.rs
fn main(){

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

Eof
```

## w/o copy

```bash
cat << EoF > ./examples/matching_on_option.rs
fn main(){
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

// cargo fmt -- --emit=files ./examples/matching_on_option.rs
// cargo run --example matching_on_option

EoF
clear
```

## rest

```bash
cat << EoF > ./examples/option_generic_type.rs
Eof
```

## test

```bash
cat << EOF > ./examples/test.rs

EoF
```

## test 2

```bash
cat << EoF > ./examples/test_two.rs

Eof

```
