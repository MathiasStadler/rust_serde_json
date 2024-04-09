use serde::{Deserialize, Serialize};
use serde_json::Result;
#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    vip: bool,
}
fn create_person() -> Result<str,serde_json::Error> {
    let p = Person {
        name: "John Doe".to_string(),
        age: 43,
        vip: true,
    };
    let j = serde_json::to_string(&p)?;
    Ok(j)
}

fn main() {
    let _result = create_person();
    let _str = match _result {
        Ok(_str) => &_str,
        Err(e) => return Err(e),
    };

    println!("{}", str);

    Ok(())
}
