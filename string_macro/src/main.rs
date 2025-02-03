fn main() {
    let num1 = 10.0;
    let num2 = 12.0;
    let division = num1 / num2;
    
    let name = "Arjun";

    // Display in 3 decimal value
    println!("Division is: {:.3}", division);

    // Display with spaces
    println!("Division is: {:8.3} ", division);

    // Display with infront zeros
    println!("Division is: {:08.3}", division);

    // Display mutiple variables
    println!("First number is: {}\nSecond number is: {} ", num1, num2);

    // Change position
    println!("First  number is: {1}\nSecond number is: {0} ", num1, num2);
    
    // Display with string letarals
    println!("My name is: {name}");

    // Display on single line
    print!("Division is: {}", division);
    
    
}
