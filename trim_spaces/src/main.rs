fn main() {
    let trim = trim_spaces("hello world");
    println!("With trim: {} ", trim);
}

fn trim_spaces(s: &str) -> &str {
    // Locate the first non space character
    let mut start = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }
    }

    // Search in reverse to locate the last non-space character
    let mut end = 0;
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }
    &s[start..end]
}
