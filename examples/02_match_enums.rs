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

fn main() {
    let _this = Example::This;
    let _that = Example::That;

    println!("Example::This contains: {:?}", _this);
    println!("Example::That contains: {:?}", _that);

    matcher(Example::This);
    matcher(Example::That);
}
// cargo run --example 02_match_enums
