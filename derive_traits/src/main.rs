#[derive(PartialEq, PartialOrd)]

struct Product {
    title: String,
    price: f64,
}

fn main() {
    let product1 = Product {
        title: String::from("iPhone 13"),
        price: 40000.0,
    };

    let product2 = Product {
        title: String::from("iPhone 13 Pro"),
        price: 70000.0,
    };
    println!("product1 === product2 {} ", product1 == product2);
    println!("procuct1 > product2 {} ", product1 > product2);
}
