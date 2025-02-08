

fn main() {

    let num = 1u8;

    let result = match num {
        0 => "zero",
        1 => "one",
        2 => "two",
        _=> {
            println!("Number not found! {} ", num);
            "something else"
        }
    };

    println!("The search result is: {} ", num);
}
