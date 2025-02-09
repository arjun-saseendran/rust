use std::fs;


fn main() {
let content = fs::read_to_string("hell.txt").expect("Item not found!");
    println!("The content is: {:?}",content);
}
