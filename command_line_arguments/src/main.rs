use std::env;

fn main() {

    if env::args().len() <= 2 {
        println!("Program requires as least two arguments!");
        return;
    }

    for (index, argument) in env::args().enumerate() {
        println!("Argument {} is: {} ",index, argument);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("The second argument is: {} ",arg2);
}
