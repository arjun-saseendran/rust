use std::fmt;

fn display() -> impl fmt::Display {
    " one"
}

fn main() {
    println!("Output is: {} ", display());
}
