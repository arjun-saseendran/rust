fn main() {
    // Declare multidimentional array
    let numbers = [[1, 2, 3], [4, 5, 6]];

    // Get second array second value
    let second_array_second_value = numbers[1][1];

    println!(
        "Second array second value is: {} ",
        second_array_second_value
    );

    // Declare three dimentional array
    let three_dimentional_array_numbers: [[[i32; 100]; 20]; 5];

    // Assign values
    three_dimentional_array_numbers = [[[0; 100]; 20]; 5];
}
