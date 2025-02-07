#[derive(Debug)]

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64)
}

fn main() {
    
    let shape = Shape::Rectangle(1.2, 3.4);
    println!("The shape is: {:?} ", shape);

    
    match shape{
        Shape::Circle(r) => println!("Circle with radius: {} ",r),
        Shape::Rectangle(w, h) => println!("Rectangle with width: {} height: {}",w, h),
        Shape::Triangle(s1, s2, s3) => println!("Triangle with side one {} side two {} side three {}", s1, s2, s3)

    }
}
