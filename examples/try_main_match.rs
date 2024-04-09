// FROM HERE
// https://stackoverflow.com/questions/36362020/what-is-unwrap-in-rust-and-what-is-it-used-for


fn try_main() -> std::io::Result<()> {
    let entries = std::fs::read_dir("/home")?;

    for entry in entries {
        println!("Name: {}", entry?.path().display());

    }
    Ok(())
}

// FROM HERE
// https://stackoverflow.com/questions/71267423/handling-a-response-from-a-result-using-match
fn main() {
    let result = try_main();

    match result {
        Ok(v)=> println!("Ok arm {:?}", v),
        Err(e) => println!("Err arm => Error: {:?}", e)
    }

    
    // if let Err(e) = res {
    //     println!("main => Error: {}", e);
    // }
}
