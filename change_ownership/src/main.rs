fn main() {
    let name = String::from("Hello");
   let name = add(name);
    println!("The number is: {} ",name);
}

fn add(name: String)-> String{
    
    println!("Send name is: {} ",name);
    // let new_name = String::from("Hi");
    // new_name
    name
}
