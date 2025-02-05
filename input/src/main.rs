use std::io;
fn main() {
    let mut buffer = String::new();
    println!("Enter your name:");
    io::stdin().read_line(&mut buffer);
    println!("Hello, {} ",buffer);
}
