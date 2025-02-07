#[derive(Debug)]

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

fn main() {
    let shape1 = Shape::Rectangle(1.3, 3.4);
    println!("The new shape is: {:?} ", shape1);
}
