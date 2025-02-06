struct People {
    name: String,
    age: u8,
    place: String,
}

impl People {
    fn new(name: &str, age: u8, place: &str) -> People {
        People {
            name: String::from(name),
            age: age,
            place: String::from(place),
        }
    }
}

fn main() {
    let mut person = People::new("C", 90, "USA");

    println!("The new person name is: {} ", person.name);
}
