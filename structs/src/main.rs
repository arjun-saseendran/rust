
#[derive(Debug)]

struct People {
    name: String,
    age: u8,
    place: String
}

fn main() {
    let mut person = People {
        name: String::from("Rust"),
        age: 15,
        place: String::from("Mozilla")
    };
    println!("My name is: {} ",person.name);
    person.name = String::from("Python");
    println!("The name changed to: {} ",person.name);

}
