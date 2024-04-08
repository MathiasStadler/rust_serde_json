use serde::{Deserialize, Serialize};
use serde_json::Result;
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    vip: bool,
}
fn create_person() -> Result<String> {
    let p = Person {
        name: "John Doe".to_string(),
        age: 43,
        vip: true,
    };
    let j = serde_json::to_string(&p)?;
    Ok(j)
}

fn main() {
    let _= create_person();
}
