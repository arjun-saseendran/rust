use std::fs;



fn main() {
let result = fs::read_to_string("hello.txt");
let data = match result {
        Ok(message) => message,
        Err(error) => String::from("Item not found!")
    };


    println!("The data is {:?}", data);
}
