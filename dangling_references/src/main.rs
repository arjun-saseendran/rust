fn main() {
    let name = display_name();
    println!("My name is: {} ", name);
}

fn display_name() -> String {
    let new_name = String::from("Hello");
    new_name
}
