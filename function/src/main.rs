// Declare main function
fn main() {
    // Call function
    display();

    // Call add function
    add(5, 5);

    // Call multiply function
    let product = multiply(5);

    println!("The product is: {} ", product);

    // Call divide function
    let div = divide(5);

    println!("The divide is: {:?} ", div);

    // Unit data type function
    hello_world();

    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

// Function without argument without return value
fn display() {
    println!("Welcome to Rust functions");

    // Call function with argument
    number(5);
}

// Function with argument without return value
fn number(num: i32) {
    println!("The number is: {} ", num);
}

// Sum function with two arguments
fn add(num1: u8, num2: u8) {
    let sum = num1 + num2;
    println!("The answer is: {}", sum);
}

// Function with argument with return value
fn multiply(num: u8) -> u8 {
    num * num
}

// Function with argument multiple return value
fn divide(num: u8) -> (u8, u8) {
    return (num, num / num);
}

// Function without return and unit data type
fn hello_world() -> () {
    println!("hello world!");
}

// Convert celsius to fahrenheit
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    let fahrenheit = (1.8 * celsius) + 32.0;
    return fahrenheit;
}
