fn main() {
    let message = String::from("Greetings form Rust");
    let first_word = get_first_word(&message[5..]);
    println!("The first word is: {} ", first_word);
}

fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }
    &s
}
