use std::any;
use std::fmt;

fn print_type<T: fmt::Debug>(item: T) {
    println!("{:?} is {} ", item, any::type_name::<T>());
}

fn main() {
    print_type(1);
    print_type(1.5);
    print_type("one");
    print_type([1]);
}
