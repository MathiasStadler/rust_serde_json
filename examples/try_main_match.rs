// FROM HERE
// https://stackoverflow.com/questions/36362020/what-is-unwrap-in-rust-and-what-is-it-used-for

fn try_main() -> std::io::Result<()> {
    let entries = std::fs::read_dir("/home")?;

    for entry in entries {
        println!("Directory => {}", entry?.path().display());
    }
    Ok(())
}

#[allow(dead_code)]
fn try_main_dir(dir: &str) -> std::io::Result<()> {
    let entries = std::fs::read_dir(dir)?;

    for entry in entries {
        println!("Directory => {}", entry?.path().display());
    }
    Ok(())
}

// FROM HERE
// https://stackoverflow.com/questions/71267423/handling-a-response-from-a-result-using-match
fn main() {
    let result = try_main();

    match result {
        Ok(v) => println!("Ok arm {:?}", v),
        Err(e) => println!("Err arm => Error: {:?}", e),
    }
}

// FROM HERE
// https://doc.rust-lang.org/book/ch11-01-writing-tests.html

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn larger_can_hold_smaller() {
    //     let larger = Rectangle {
    //         width: 8,
    //         height: 7,
    //     };
    //     let smaller = Rectangle {
    //         width: 5,
    //         height: 1,
    //     };

    //     assert!(larger.can_hold(&smaller));
    // }

    #[test]
    fn try_main() {
        let result = try_main_dir("/home/user");

        match result {
            Ok(v) => println!("Ok arm {:?}", v),
            Err(e) => println!("Err arm => Error: {:?}", e),
        }
    }
}
