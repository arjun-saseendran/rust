fn main() {
    // Declare mutable array
    let mut letters = ['a', 'b', 'c'];

    // Assign a value to a array's specific position
    letters[1] = 'x';

    // Assign specific postion value to a variable
    let second_letter = letters[1];

    println!(
        "Array seond position value after the updation is: {} ",
        second_letter
    );

    // Declare a numbers array
    let numbers: [i32; 5];

    // Repeat expression
    numbers = [0; 5];

    println!("Last number of the array is: {} ", numbers[4]);

    // Set index size
    let index: usize = numbers.len();

    println!("The length of the array is: {} ", index);
}
