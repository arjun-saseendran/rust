fn main() {
    let a = true;
    let b = false;
    println!("a is: {} and b is: {} ", a,b);
    println!("NOT a is: {} ", !a);
    println!("a AND b is: {} ", a & b);
    println!("a OR b is: {} ", a | b);
    println!("a XOR b is: {} ", a ^ b);
    
    let mut c = (a ^ b) | (a & b);
    println!("c is: {}",c);
    
    c = (a ^ b) || (a & b);
    
    println!("c is: {}", c);
    
    c = (a ^ b) && panic!();
    println!("c is: {} ",c);
    
    
}
