fn main() {
    let countdown = [4, 1, 3, 5,2];
    let number = countdown.get(5);
    let number = number.unwrap_or(&0) + 1;
    println!("The number is: {} ", number);
}
