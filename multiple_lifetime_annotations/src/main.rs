fn big_number<'a, 'b>(num1: &'a str, num2: &'b str) -> &'a str {
    if num1.len() > num2.len() {
        num1
    } else {
        num1
    }
}

fn main() {
    let result;
    let test = String::from("Testing");
    {
        let testing2 = String::from("Testing two");
        result = big_number(&test, &testing2);
    }
    println!("The result is: {} ", result);
}
