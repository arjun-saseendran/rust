struct People {
    name: String,
    age: u8,
    place: String,
}

impl People {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_age(&mut self, age: u8) {
        self.age += age;
    }
}

fn main() {
    let mut person = People {
        name: String::from("Ruby"),
        age: 29,
        place: String::from("Japan"),
    };

    let person_name = person.get_name();
    println!("The person name is: {} ", person_name);

    person.add_age(15);

    println!("The person new age is: {} ", person.age);
}
