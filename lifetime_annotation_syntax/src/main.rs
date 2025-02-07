fn product<'a>(title1: &'a str, title2: &'a str) -> &'a str {
    if title1.len() > title2.len() {
        title1
    } else {
        title2
    }
}

fn main() {
    let result;
    let title1 = String::from("iPhone 13");
    let title2 = String::from("iPhone 13 Pro");
    result = product(&title1, &title2);
    println!("The longest string length is: {} ", result);
}
