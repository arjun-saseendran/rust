fn main() {
    // Declare tuples
    let data = (1, 1.5, 'a');

    // Get first position data
    let first_data = data.0;

    println!("The first data is: {} ", first_data);

    // Assign data type
    let mut new_data: (u8, f32, char) = (10, 3.5, 'a');

    // Destructing tuples
    let (a, b, c) = new_data;

    println!("a is: {} b is: {} c is: {} ", a, b, c);

    // Set data
    new_data.2 = 'b';

    println!("c is: {} ", c);
}
