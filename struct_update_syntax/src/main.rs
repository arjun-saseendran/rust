#[derive(Debug)]
#[derive(Clone)]

struct People {
    name: String,
    age: u8,
    place: String
}


fn main() {
    
    let mut person1 = People{
        name: String::from("Javascript"),
        age: 30,
        place: String::from("USA")
    };

    let mut person2 = People{
        ..person1.clone()
    };

    person2.name = String::from("Java");

    println!("My name is: {:?} ",person1);
    println!("My name is: {:?} ",person2);

}
