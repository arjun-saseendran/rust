use std::collections::HashMap;


fn main() {
let mut days= HashMap::new();
days.insert("Monday", 1);
days.insert("Tuesday", 2);
days.insert("Wednesday", 3);
println!("Days are: {:?} ", days);
days.insert("Monday", 2);
println!("After updating Monday: {:?} ", days);
days.entry("Thrusday").or_insert(4);
println!("After adding Thursday: {:?}, ", days);
let num = days.entry("Tuesday").or_insert(2);
*num +=1;
println!("Updated days are: {:?} ", days);

let new_data = days.get("Tuesday");
println!("The new data is: {:?} ", new_data);
}
