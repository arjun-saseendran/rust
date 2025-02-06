struct Product {
    title: String,
    price: f64,
}

struct Order {
    order_id: String,
    quantity: u8,
}

trait Details {
    fn describe(&self) -> String;
}

impl Details for Product {
    fn describe(&self) -> String {
        format!(
            "The product is: {} and The price is: {} ",
            self.title, self.price
        )
    }
}

impl Details for Order {
    fn describe(&self) -> String {
        format!("The order id is: {} and The quantity id is: {} ", self.order_id, self.quantity)
    }
}

fn main() {
    let product1 = Product {
        title: String::from("iPhone 13"),
        price: 40000.0,
    };

    let order1 = Order {
        order_id: String::from("1"),
        quantity: 1,
    };

    println!("{} ", product1.describe());
    println!("{} ", order1.describe());
}
