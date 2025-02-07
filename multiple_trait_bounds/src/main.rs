use std::fmt;

fn compare_display<T, U>(num1: T, num2: U)
where T: fmt::Display + PartialEq + From<U>,
    U: fmt::Display + PartialEq + Copy {
        if num1 == T::from(num2){
            println!("{} is equal to {} ",num1, num2);
        }else {
            println!("{} not equal to {} ", num1, num2);
        }
    } 

fn main() {
    compare_display(1.0, 1);
    compare_display(1.1, 1);
    
}
