fn main() {
    let nums = [3, 1, 4, 2, 5];
    let num = nums.get(2);
    let num = match num {
        Some(num) => num + 1,
        None => 0,
    };

    println!("The number is: {} ", num);
}
