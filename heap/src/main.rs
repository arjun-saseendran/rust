fn main() {
    let outer_text:String;
    let mut outer_number:i32 = 0;
    
    {
        let mut inner_text = String::from("Hello");
        let mut inner_number = 1;
        outer_text = inner_text.clone();
        inner_number = outer_number;
        inner_text.clear();
        println!("Inner text is: {} ", inner_text);
        println!("Inner number is: {} ", inner_number);
    }
    
println!("Outer text is: {} ", outer_text);
println!("Outer number is: {} ", outer_number);


}
