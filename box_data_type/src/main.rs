use std::mem;

struct People {
    name: String,
    age: u8,
    place: String,
}

fn main() {
    let person1 = People {
        name: String::from("Google"),
        age: 27,
        place: String::from("California"),
    };

    println!(
        "person1 size on stack: {} bytes",
        mem::size_of_val(&person1)
    );

    let boxed_person1: Box<People> = Box::new(person1);

    println!(
        "box person1 size on stack: {} bytes ",
        mem::size_of_val(&boxed_person1)
    );

    println!(
        "box person1 size on heap: {} bytes",
        mem::size_of_val(&*boxed_person1)
    );

    let unboxed_person1: People = *boxed_person1;
    println!(
        "unboxed person1 size on stack: {} bytes ",
        mem::size_of_val(&unboxed_person1)
    );
}
