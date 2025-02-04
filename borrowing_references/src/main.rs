fn main() {
    let name = String::from("Hello");
    let length = display_name(&name);
    println!("My name is: {} and length is: {} ",name,length);
}

fn display_name(name: &String)->usize{
    println!("Send name is: {} ",name);
    let length = name.len();
    length
}
