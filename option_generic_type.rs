let a_str: Option<&str> = Some("a str");
let a_string: Option<String> = Some(String::from("a String"));
let a_float: Option<f64> = Some(1.1);
let a_vec: Option<Vec<i32>> = Some(vec![0, 1, 2, 3]);

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

fn main(){

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

