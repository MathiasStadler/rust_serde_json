# [FROM HERE](http://saidvandeklundert.net/learn/2021-09-01-rust-option-and-result/)

## the enum in Rust

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
