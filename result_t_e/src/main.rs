use std::fs;


fn main() {
let content = fs::read_to_string("hello.txt");
    println!("The content is: {:?}",content);
}
