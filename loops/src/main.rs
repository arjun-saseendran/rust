fn main() {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    let array = [1, 2, 3];
    loop {
        if i == 5 {
            break;
        }
        i += 1;
        println!("i is: {} ", i);
    }

    let result = loop {
        if i == 5 {
            break i * 10;
        }
        i += 1;
    };

    println!("The result is: {}", result);

    while j < 5 {
        j += 1;
        println!("j is: {} ", j);
    }

    while k < array.len() {
        println!("The array elements are: {} ", array[k]);
        k += 1;
    }

    let name = ['A', 'r', 'j', 'u', 'n'];

    for letter in name {
        println!("Letter is: {} ", letter);
    }

    for (index, &letter) in name.iter().enumerate() {
        println!("Index of: {} letter is: {}", index, letter);
        if letter == 'u' {
            break;
        }
    }

    for numbers in 0..5 {
        println!("numbers: {} ", numbers);
    }

    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    for row in matrix {
        for num in row {
            print!("{}\t", num);
        }
        println!();
    }

    let nums = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    max = nums[0];
    min = nums[0];
    mean = 0.0;

    for &n in nums.iter() {
        if n > max {
            max = n;
        } else if n < min {
            min = n;
        }

        mean += n as f64;
    }

    mean /= nums.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}
