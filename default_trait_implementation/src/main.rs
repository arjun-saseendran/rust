struct Product {
    title: String,
    price: f64,
}

struct Order {
    order_id: String,
    quantity: u8,
}

trait Details {
    fn describe(&self) -> String {
        String::from("The product is avilable")
    }
}

impl Details for Product {}

impl Details for Order {
    fn describe(&self) -> String {
        format!(
            "The order id is: {} and the quantity is: {} ",
            self.order_id, self.quantity
        )
    }
}

fn main() {
    let product2 = Product {
        title: String::from("iPhone 13 Pro"),
        price: 70000.0,
    };

    let order2 = Order {
        order_id: String::from("2"),
        quantity: 1,
    };

    println!("{} ", product2.describe());
    println!("{} ", order2.describe());
}
