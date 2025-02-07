use std::fmt;

struct Product {
    title: String,
    price: f64,
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Product is: {} and the price is: {} ",
            self.title, self.price
        )
    }
}

fn main() {
    let product3 = Product {
        title: String::from("iPhone 14"),
        price: 50000.0
    };
    println!("{} ",product3);
}
