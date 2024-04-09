// FROM HERE
// https://stackoverflow.com/questions/59710923/how-can-i-return-an-error-from-serde-in-a-function-that-returns-result-erro

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Mine {
    ab: u8,
    ac: u8,
}
// #[macro_use]
// extern crate serde_derive;

fn do_match() -> Result <(), serde_json::error::Error> {
    serde_json::from_str::<Mine>("test").map(|_| println!("Okay"))
}

fn main() {
    if do_match().is_ok() {
        println!("Success");
    }
}