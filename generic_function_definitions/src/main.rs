


fn main() {
    println!("The biggest number is: {} ",big_number(0,2));
}



fn big_number<T: PartialOrd>(num1: T, num2: T) -> T{
    
    if num1 > num2 {
        num1
    }else {
        num2
    }
    
}