pub fn main() {
    #[derive(Debug)]
    enum Example {
        This,
        That,
    }

    let _this = Example::This;
    let _that = Example::That;

    println!("this => {:?}", _this);
    println!("that => {:?}", _that);
}
// cargo run --example 01_simple_enums
