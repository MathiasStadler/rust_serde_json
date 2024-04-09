# [FROM HERE](http://saidvandeklundert.net/learn/2021-09-01-rust-option-and-result/)

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
