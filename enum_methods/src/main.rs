use core::hash;

#[derive(Debug)]

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
            Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
            Shape::Triangle(one, two, three) => one + two + three,
        }
    }
}

fn main() {
    let shape1 = Shape::Rectangle(1.2, 3.4);
    println!("The shape is: {:?} ", shape1);

    let peri = shape1.perimeter();
    println!("The perimeter is: {} ", peri);
}
