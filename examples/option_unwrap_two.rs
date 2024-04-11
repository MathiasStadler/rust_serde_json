// FROM HERE
// https://github.com/saidvandeklundert/LearningRust/blob/master/blog/option-and-result/src/main.rs


fn main(){
let something: Option<&str> = Some("Something");
        let unwrapped = something.unwrap(); // "Something"
        println!("{:?}\n{:?}", something, unwrapped);
        let nothing: Option<&str> = None;
        // uncommenting the next line will cause a panic:
        //nothing.unwrap();

}
// cargo run --example option_unwrap_two

