use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Enter a number:");
    io::stdin().read_line(&mut buffer);
    let number:i32 = buffer.trim().parse().unwrap();
    println!("Entered number is: {} ",number);

}
