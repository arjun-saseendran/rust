struct Product<'a> {
    title: &'a str,
}

impl<'a, 'b> Product<'a> {
    fn send_message(&'a self, message: &'b str) -> &'b str {
        println!("Transmitting message: {} ", message);
        message
    }
}

fn main() {
    let product1 = Product { title: "iPhone 13" };
    let sender = product1.send_message("Product test passed!");
    println!("Test by: {} ", sender);
}
