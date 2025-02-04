fn main() {
    let name = String::from("Welcome to Rust");
    println!("My name is: {} ",name);
    
    let last_word = &name[5..5+5];
    // let last_word = &name[5..];
    println!("The last word is: {} ",last_word);
    
    let numbers = [1, 2, 3, 4, 5];
    let some_numbers: &[i32] = &numbers[..2];
    println!("Some numbers are: {:?} ",some_numbers);
}
