fn main() {
    let mut name = String::from("Hello");
    let length = display_name(&mut name);
    println!("My name is: {} and length is: {} ",name, length);
}

fn display_name(name: &mut String)-> usize{
    println!("Send name is: {} ", name);
    name.push_str("Hi");
    let length = name.len();
    length
    
}
