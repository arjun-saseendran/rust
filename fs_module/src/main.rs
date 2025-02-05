use std::fs;
use std::io::prelude::*;

fn main() {
    let data = fs::read_to_string("note.txt").unwrap();
    println!("The read result is: \n{} ",data);

    for line in data.lines() {
        println!("Line is: {} ",line);
    }
    
    let data = fs::read("note.txt").unwrap();
    println!("The note is: {:?} ",data);

    let mut text = String::new();
    text.push_str("hello world");

    fs::write("text.txt", text);

    let mut file = fs::OpenOptions::new().append(true).open("text.txt").unwrap();
    file.write(b"\nWelcome to Rust!");

}
