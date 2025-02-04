fn main() {
    let a = 10;
    let b = 40;
    let c = 20;

    // If statement
    if a > c {
        println!("The a is big number");
    } else if b > a {
        println!("The b is big number");
    }

    let status = if c > a { 1 } else { 0 };
    println!("The status is: {} ", status);
}
