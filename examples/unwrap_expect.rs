// FROM HERE
// https://doc.rust-lang.org/rust-by-example/error/option_unwrap.html

// The adult has seen it all, and can handle any drink well.
// All drinks are handled explicitly using `match`.
fn give_adult(drink: Option<&str>) {
    // Specify a course of action for each case.
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary."),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink? Oh well."),
    }
}

// Others will `panic` before drinking sugary drinks.
// All drinks are handled implicitly using `unwrap`.
fn drink(drink: Option<&str>) {
    // `unwrap` returns a `panic` when it receives a `None`.
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }

    println!("I love {}s!!!!!", inside);
}

fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}

/*
export FILE_NAME="unwrap_expect.rs"
export FILE_DIR_NAME="./examples"
git add $FILE_DIR_NAME/$FILE_NAME
git commit --all --message="add BEFORE housekeeping => $FILE_DIR_NAME/$FILE_NAME"
git push
cargo clippy --fix
cargo clippy --fix --examples
cargo fmt -- --emit=files
git commit --all --message="add AFTER housekeeping => $FILE_DIR_NAME/"
git push
cargo run --example $(echo $FILE_NAME | cut -d . -f 1)
*/
